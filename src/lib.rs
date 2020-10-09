use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;
use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, Promise, PromiseResult, PublicKey,
};
use std::convert::TryInto;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 红包的领取信息结构
#[derive(Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ClaimInfo {
    pub user: AccountId,  // 领取者账户
    pub amount: Balance, // 领取到的红包价值
}

/// 红包信息结构
#[derive(Clone)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RedInfo {
    pub owner: AccountId,  // 红包的发送人
    pub mode: u8, // 红包模式,随机红包1;均分红包0
    pub count: u128, // 红包数量
    pub slogan: String, // 祝福词
    pub balance: Balance, // 总金额
    pub remaining_balance: u128, // 红包剩余金额
    pub claim_info: Vec<ClaimInfo>,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct RedBag {
    // 红包库
    pub red_info: Map<PublicKey, RedInfo>, 
    // 记录用户发送的红包
    pub sender_redbag: Map<AccountId, Vec<PublicKey>>,
    // 记录用户领取的红包
    pub receiver_redbag: Map<AccountId, Vec<PublicKey>>,
}

/// Access key allowance for redbag keys, 
/// take it as additional fee used by creation new account.
const ACCESS_KEY_ALLOWANCE: u128 = 1_000_000_000_000_000_000_000_000;

/// Gas attached to the callback from account creation.
pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: u64 = 20_000_000_000_000;

/// Indicates there are no deposit for a callback for better readability.
const NO_DEPOSIT: u128 = 0;

#[ext_contract(ext_self)]
pub trait ExtRedBag {
    /// Callback after creating account and claiming redbag.
    fn on_account_created_and_claimed(&mut self, amount: U128) -> bool;
}

fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}

#[near_bindgen]
impl RedBag {

    ///  发红包功能
    #[payable]
    pub fn send_redbag(
        &mut self, 
        public_key: Base58PublicKey, 
        count: u128, 
        mode: u8, 
        slogan: String) -> Promise {

        assert!(
            env::attached_deposit() > count * ACCESS_KEY_ALLOWANCE,
            "Attached deposit must be greater than count * ACCESS_KEY_ALLOWANCE"
        );

        let owner = env::signer_account_id();
        let pk: PublicKey = public_key.clone().into();
        assert!(self.red_info.get(&pk).is_none(), "The public_key already exists");

        // 初始化红包信息并记录
        let new_red_info = RedInfo {
            owner: owner.clone(),
            mode,
            count,
            slogan,
            balance: env::attached_deposit() - count * ACCESS_KEY_ALLOWANCE,
            remaining_balance: env::attached_deposit() - count * ACCESS_KEY_ALLOWANCE,
            claim_info: Vec::new(),
        };
        self.red_info.insert(&pk, &new_red_info);
        
        // 更新账户的发红包记录
        let mut relation_vec = self.sender_redbag.get(&owner).unwrap_or(Vec::new());
        relation_vec.push(pk.clone());
        self.sender_redbag.insert(&owner, &relation_vec);

        Promise::new(env::current_account_id()).add_access_key(
            pk,
            ACCESS_KEY_ALLOWANCE,
            env::current_account_id(),
            b"create_account_and_claim,claim,revoke".to_vec(),
        )
    }

    fn claim_redbag(&mut self, pk: PublicKey, account_id: AccountId) -> Balance {
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");

        // 查看红包剩余数量是否可被领取
        let mut rb = &mut redbag.unwrap();
        assert!(rb.claim_info.len() < rb.count as usize, 
            "Sorry, the redbag has been claimed out.");
        assert!(rb.remaining_balance != 0, 
            "Sorry, the redbag has been revoked.");
        // 判断用户是否领取过
        assert!(rb.claim_info.iter().filter(|x| x.user == account_id).count() == 0, 
            "Sorry, you have claimed this redbag before.");
        // 领取红包 如果是最后一个领取人，则拿走所有
        let amount = if rb.claim_info.len() == rb.count as usize - 1 { 
                rb.remaining_balance 
            } else { 
                self.random_amount(rb.remaining_balance)
        };
        
        // 更新红包记录
        rb.remaining_balance -= amount;
        let ci = ClaimInfo {
            user: account_id.clone(),
            amount,
        };
        rb.claim_info.push(ci);
        self.red_info.insert(&pk, &rb);
        // 更新领取人记录
        let mut receiver_record = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        receiver_record.push(pk.clone());
        self.receiver_redbag.insert(&account_id, &receiver_record);
        
        amount
    }

    /// 创建新用户同时领取红包
    pub fn create_account_and_claim(
        &mut self,
        new_account_id: AccountId,
        new_public_key: Base58PublicKey) -> Promise {

        let pk = env::signer_account_pk();
        let amount = self.claim_redbag(pk.clone(), new_account_id.clone());

        Promise::new(new_account_id)
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(amount)
            .then(ext_self::on_account_created_and_claimed(   
                amount.into(),
                &env::current_account_id(),
                NO_DEPOSIT,
                ON_CREATE_ACCOUNT_CALLBACK_GAS,
            ))
    }

    /// 领取红包
    pub fn claim(&mut self, account_id: AccountId) -> Promise {
        let pk = env::signer_account_pk();

        let amount = self.claim_redbag(pk.clone(), account_id.clone());

        Promise::new(account_id).transfer(amount)
    }

    /// 红包所有人撤回对应public_key的红包剩余金额
    pub fn revoke(&mut self, public_key: Base58PublicKey) -> Promise {
        let pk: PublicKey = public_key.clone().into();
        let account_id = env::signer_account_id();
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");
        // 查看红包剩余数量和金额是否足够撤回
        let mut rb = &mut redbag.unwrap();
        assert!(rb.owner == account_id, 
            "Sorry, Only redbag owner can revoke.");
        assert!(rb.claim_info.len() < rb.count.try_into().unwrap(), 
            "Sorry, the redbag has been claimed out.");
        // 红包剩余
        let amount: Balance = rb.remaining_balance;
        // 更新红包记录
        rb.remaining_balance = 0;
        let ci = ClaimInfo {
            user: account_id.clone(),
            amount,
        };
        rb.claim_info.push(ci);
        self.red_info.insert(&pk, &rb);
        // 更新领取人记录
        let mut receiver_record = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        receiver_record.push(pk.clone());
        self.receiver_redbag.insert(&account_id, &receiver_record);

        Promise::new(account_id).transfer(amount)
    }

    /// ******************** view functions ***************************
    /// 查询红包领取详情
    pub fn show_claim_info(self, public_key: Base58PublicKey) -> String {
        let pk = public_key.into();
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");
        let rb = &redbag.unwrap();

        let ci_json: Vec<_> = rb.claim_info.iter().map(
            |x| format!("{{\"account\":\"{}\", \"amount\":{}}}", x.user, x.amount)
        ).collect();
        let recvs_json = format!("[{}]", ci_json.join(","));

        format!("{{\"owner\":\"{}\", \"count\":{}, \"mode\":{}, \"slogan\":\"{}\",\"list\":\"{}\"}}", 
            rb.owner, rb.count, rb.mode, rb.slogan, recvs_json)
    }

    /// 查询用户所发的所有红包
    pub fn show_redbag(self, account_id: AccountId) -> Vec<Base58PublicKey> {
        let relation_vec = self.sender_redbag.get(&account_id).unwrap_or(Vec::new());
        relation_vec.iter().map(|x| x.clone().try_into().unwrap()).collect()
    }

    /// 生成随机, 255个层级 total_amount * share_rate / u8::max_value().into()
    fn random_amount(&self, total_amount: u128) -> u128 {
        let u8_max_value: u128 = u8::max_value().into();
        let min_share: u128 = total_amount / u8_max_value;

        // 获取随机比率
        let random_seed = env::random_seed();
        let mut share_rate: u8 = random_seed.iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));

        // 限制过大或过小的比率在2%到60%之间
        if share_rate < 5_u8 {
            share_rate = 5;
        } else if share_rate > 153 {
            share_rate = 153;
        }

        min_share.wrapping_mul(share_rate.into())
    }

    /// Returns the balance associated with given key.
    pub fn get_key_balance(&self, key: Base58PublicKey) -> U128 {
        let pk = key.clone().into();
        let redbag_info = self.red_info.get(&pk).unwrap();
        redbag_info.remaining_balance.into()
    }

    /// callback after execution `create_account_and_claim`.
    pub fn on_account_created_and_claimed(&mut self, amount: U128) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Callback can only be called from the contract"
        );
        let creation_succeeded = is_promise_success();
        if creation_succeeded {  // TODO: 红包逻辑中无需删除key, 但要补记相关信息
            // Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
            assert_eq!(1, 1, "Nop");
        } else {
            // In case of failure, put the amount back.
            // TODO: 失败的情况下，回退资金及相关结构信息的更改 
            // self.accounts.insert(&env::signer_account_pk(), &amount.into());
            assert_eq!(1, 1, "Nop");
        }
        creation_succeeded
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, BlockHeight, PublicKey, VMContext};
}
