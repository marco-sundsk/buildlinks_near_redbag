use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base58PublicKey, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::wee_alloc;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight, Gas, Promise, PromiseResult,
    PublicKey,
};
use std::collections::HashSet;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

use std::convert::TryInto;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Access key allowance for redbag keys,
/// take it as additional fee used by creation new account.
const ACCESS_KEY_ALLOWANCE: Balance = 100_000_000_000_000_000_000_000;

/// the minimum balance that an account must contain to maintain state fee.
/// 0.1 NEAR
const MIN_REDBAG_SHARE: Balance = 100_000_000_000_000_000_000_000;

/// Gas attached to the callback from account creation.
pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: Gas = 20_000_000_000_000;

/// Indicates there are no deposit for a callback for better readability.
const NO_DEPOSIT: Balance = 0;

/// one claim info
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct ClaimInfo {
    pub user: AccountId, // 领取者账户
    pub amount: Balance, // 领取到的红包价值
    pub height: BlockHeight,
    pub ts: u64,
}

/// one claim info used for return
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableClaimInfo {
    pub user: AccountId,
    pub amount: U128,
    pub height: U64,
    pub ts: U64,
}

/// describe a redbag
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RedInfo {
    pub owner: AccountId,           // 红包的发送人
    pub mode: u8,                   // 红包模式,随机红包1; 均分红包0
    pub count: u8,                  // 红包数量
    pub slogan: String,             // 祝福词
    pub balance: Balance,           // 总金额
    pub remaining_balance: Balance, // 红包剩余金额
    pub height: BlockHeight,
    pub ts: u64,
    pub claim_info: Vec<ClaimInfo>,
}

/// detail info about a redbag used for return 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableRedDetail {
    pub owner: AccountId,
    pub mode: u8,
    pub count: u8,
    pub balance: U128,
    pub remaining_balance: U128,
    pub height: U64,
    pub ts: U64,
    pub claim_info: Vec<HumanReadableClaimInfo>,
}

// brief info about a redbag used for return
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableRedBrief {
    pub owner: AccountId,
    pub id: Base58PublicKey,
    pub mode: u8,
    pub count: u8,
    pub balance: U128,
    pub remaining_balance: U128,
    pub received_count: u8,
    pub height: U64,
    pub ts: U64,
}

// Brief of recv info for a receiver
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableRecvBrief {
    pub id: Base58PublicKey,
    pub balance: U128,
    pub height: U64,
    pub ts: U64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RedBag {
    // 红包库
    pub red_info: LookupMap<PublicKey, RedInfo>,
    // 记录用户发送的红包
    pub sender_redbag: LookupMap<AccountId, Vec<PublicKey>>,
    // 记录用户领取的红包
    pub receiver_redbag: LookupMap<AccountId, Vec<PublicKey>>,
}

#[ext_contract(ext_self)]
pub trait ExtRedBag {
    /// Callback after creating account and claiming redbag.
    fn on_account_created_and_claimed(&mut self, amount: U128) -> bool;
    fn on_account_created_and_claimed_ex(&mut self, 
        pk: Base58PublicKey, account_id: AccountId, 
        height: U64, amount: U128) -> bool;
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

impl Default for RedBag {
    fn default() -> Self {
        // env::panic(b"dice contract should be initialized before usage")
        Self {
            red_info: LookupMap::new(b"b".to_vec()),
            sender_redbag: LookupMap::new(b"s".to_vec()),
            receiver_redbag: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl RedBag {

    /// 发红包功能
    #[payable]
    pub fn send_redbag(
        &mut self,
        public_key: Base58PublicKey,
        count: u8,
        mode: u8,
        slogan: String,
    ) -> Promise {
        let total_aka: Balance = count as Balance * ACCESS_KEY_ALLOWANCE;
        assert!(
            env::attached_deposit() > total_aka,
            "Attached deposit must be greater than count * ACCESS_KEY_ALLOWANCE"
        );

        let owner = env::signer_account_id();
        let pk: PublicKey = public_key.clone().into();
        assert!(
            self.red_info.get(&pk).is_none(),
            "The public_key already exists"
        );

        // make sure slogan would NOT be too long
        let short_slogan;
        if slogan.len() > 32 {
            short_slogan = &slogan[..32];
        } else {
            short_slogan = &slogan[..];
        }

        // 初始化红包信息并记录
        let new_red_info = RedInfo {
            owner: owner.clone(),
            mode,
            count,
            slogan: short_slogan.to_string(),
            balance: env::attached_deposit() - total_aka,
            remaining_balance: env::attached_deposit() - total_aka,
            height: env::block_index(),
            ts: env::block_timestamp(),
            claim_info: Vec::new(),
        };
        self.red_info.insert(&pk, &new_red_info);
        // 更新账户的发红包记录
        let mut relation_vec = self.sender_redbag.get(&owner).unwrap_or(Vec::new());
        relation_vec.push(pk.clone());
        self.sender_redbag.insert(&owner, &relation_vec);

        // 添加 access key
        Promise::new(env::current_account_id()).add_access_key(
            pk,
            ACCESS_KEY_ALLOWANCE,
            env::current_account_id(),
            b"create_account_and_claim,claim".to_vec(),
        )
    }

    /// 创建新用户同时领取红包
    pub fn create_account_and_claim(
        &mut self,
        new_account_id: AccountId,
        new_public_key: Base58PublicKey,
    ) -> Promise {
        let pk = env::signer_account_pk();
        let amount = self.claim_redbag(pk.clone(), new_account_id.clone());

        // Promise::new(new_account_id)
        //     .create_account()
        //     .add_full_access_key(new_public_key.into())
        //     .transfer(amount)
        //     .then(ext_self::on_account_created_and_claimed(
        //         amount.into(),
        //         &env::current_account_id(),
        //         NO_DEPOSIT,
        //         ON_CREATE_ACCOUNT_CALLBACK_GAS,
        //     ))
        Promise::new(new_account_id.clone())
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(amount)
            .then(ext_self::on_account_created_and_claimed_ex(
                pk.try_into().unwrap(),
                new_account_id,
                env::block_index().into(),
                amount.into(),
                &env::current_account_id(),
                NO_DEPOSIT,
                ON_CREATE_ACCOUNT_CALLBACK_GAS,
            ))
    }

    /// 领取红包
    pub fn claim(&mut self, account_id: AccountId) -> U128 {
        let pk = env::signer_account_pk();

        let amount = self.claim_redbag(pk.clone(), account_id.clone());

        Promise::new(account_id).transfer(amount);

        amount.into()
    }

    /// 红包所有人撤回对应public_key的红包剩余金额
    /// 撤回视为自己领取剩余金额
    pub fn revoke(&mut self, public_key: Base58PublicKey) -> Promise {
        let pk: PublicKey = public_key.clone().into();
        let account_id = env::signer_account_id();
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");
        // 查看红包剩余数量和金额是否足够撤回
        let mut rb = &mut redbag.unwrap();
        assert!(
            rb.owner == account_id,
            "Sorry, Only redbag owner can revoke."
        );
        assert!(
            rb.claim_info.len() < rb.count as usize,
            "Sorry, the redbag has been claimed out."
        );
        assert!(
            rb.remaining_balance != 0,
            "Sorry, the redbag has already been revoked."
        );
        // 红包剩余
        let amount: Balance = rb.remaining_balance;
        // 更新红包记录
        rb.remaining_balance = 0;
        let ci = ClaimInfo {
            user: account_id.clone(),
            amount,
            height: env::block_index(),
            ts: env::block_timestamp(),
        };
        rb.claim_info.push(ci);
        self.red_info.insert(&pk, &rb);
        // 更新领取人记录
        let mut receiver_record = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        receiver_record.push(pk.clone());
        self.receiver_redbag.insert(&account_id, &receiver_record);

        Promise::new(account_id).transfer(amount)
    }

    /************************/
    /* view functions       */
    /************************/

    /// obsolete
    pub fn show_redbag(&self, public_key: Base58PublicKey) -> String {
        let pk = public_key.into();
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");
        let rb = &redbag.unwrap();

        let ci_json: Vec<_> = rb
            .claim_info
            .iter()
            .map(|x| format!("{{\"account\":\"{}\", \"amount\":{}}}", x.user, x.amount))
            .collect();
        let recvs_json = format!("[{}]", ci_json.join(","));

        format!("{{\"owner\":\"{}\", \"count\":{}, \"balance\":{}, \"remaining\":{}, \"mode\":{}, \"slogan\":\"{}\",\"list\":\"{}\"}}", 
            rb.owner, rb.count, rb.balance, rb.remaining_balance, rb.mode, rb.slogan, recvs_json)
    }

    /// 看某个红包的详情
    pub fn show_redbag_detail(&self, public_key: Base58PublicKey) ->  HumanReadableRedDetail {
        let pk = public_key.into();
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");
        let redbag_info = &redbag.unwrap();

        let cis: Vec<HumanReadableClaimInfo> = redbag_info.claim_info.iter()
            .map(|x| self.format_claiminfo(&x)).collect();

        HumanReadableRedDetail {
            owner: redbag_info.owner.clone(),
            mode: redbag_info.mode,
            count: redbag_info.count,
            balance: redbag_info.balance.into(),
            remaining_balance: redbag_info.remaining_balance.into(),
            height: redbag_info.height.into(),
            ts: redbag_info.ts.into(),
            claim_info: cis,
        }
    }

    /// obsolete
    pub fn show_send(&self, account_id: AccountId) -> Vec<Base58PublicKey> {
        let relation_vec = self.sender_redbag.get(&account_id).unwrap_or(Vec::new());
        relation_vec
            .iter()
            .map(|x| x.clone().try_into().unwrap())
            .collect()
    }

    /// 查询用户所发的所有红包
    pub fn show_send_list(&self, account_id: AccountId) -> Vec<HumanReadableRedBrief> {
        let relation_vec = self.sender_redbag.get(&account_id).unwrap_or(Vec::new());
        relation_vec
            .iter()
            .map(|x| self.redbag_brief(x))
            .collect()
    }

    // obsolete
    pub fn show_recv(&self, account_id: AccountId) -> Vec<Base58PublicKey> {
        let relation_vec = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        relation_vec
            .iter()
            .map(|x| x.clone().try_into().unwrap())
            .collect()
    }

    // 查询用户抢到的所有红包
    pub fn show_recv_list(&self, account_id: AccountId) -> Vec<HumanReadableRecvBrief> {
        let relation_vec = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());

        // using set to remove duplicate id
        let a: HashSet<_> = relation_vec.into_iter().collect();

        let mut ret: Vec<HumanReadableRecvBrief> = Vec::new();
        for id in a.iter() {
            let redbag = self.red_info.get(id).unwrap();
            for ci in &redbag.claim_info {
                if ci.user == account_id {
                    ret.push(self.recv_brief(id, ci));
                }
            }
        }

        ret
    }

    /// obsolete
    pub fn show_redbag_brief(&self, public_key: Base58PublicKey) -> HumanReadableRedBrief {
        let pk = public_key.into();
        self.redbag_brief(&pk)
    }

    /************************/
    /* callback functions   */
    /************************/

    /// callback after execution `create_account_and_claim`.
    pub fn on_account_created_and_claimed(&mut self, amount: U128) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Callback can only be called from the contract"
        );
        let creation_succeeded = is_promise_success();
        if creation_succeeded {
            // Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
            assert_eq!(1, 1, "Nop");
        } else {
            // In case of failure, put the amount back.
            // TODO: 失败的情况下，回退资金及相关结构信息的更改
            // self.accounts.insert(&env::signer_account_pk(), &amount.into());
            let amount: u128 = amount.into();
            env::log(
                format!(
                    "Create account and claim failed! related amount is {}",
                    amount
                )
                .as_bytes(),
            );
            assert_eq!(1, 1, "Nop");
        }
        creation_succeeded
    }

    pub fn on_account_created_and_claimed_ex(&mut self, 
        pk: Base58PublicKey, account_id: AccountId, 
        height: U64, amount: U128) -> bool {
            assert_eq!(
                env::predecessor_account_id(),
                env::current_account_id(),
                "Callback can only be called from the contract"
            );
            let creation_ret = is_promise_success();
            if creation_ret {
                // Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
                assert_eq!(1, 1, "Nop");
            } else {
                // In case of failure, put the amount back.
                // TODO: 失败的情况下，回退资金及相关结构信息的更改
                // self.accounts.insert(&env::signer_account_pk(), &amount.into());
                let amount: Balance = amount.into();
                let remove_ret = self.remove_recv(pk.into(), account_id, height.into(), amount);
                env::log(
                    format!(
                        "Create account and claim failed! Redbag info rolled back: {}",
                        remove_ret
                    )
                    .as_bytes(),
                );
                
            }
            creation_ret
        }

    /************************/
    /* internal functions   */
    /************************/

    fn claim_redbag(&mut self, pk: PublicKey, account_id: AccountId) -> Balance {
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");

        // 查看红包剩余数量是否可被领取
        let mut rb = &mut redbag.unwrap();
        assert!(
            rb.claim_info.len() < rb.count as usize,
            "Sorry, the redbag has been claimed out."
        );
        assert!(
            rb.remaining_balance != 0,
            "Sorry, the redbag has been revoked."
        );
        assert!(
            rb.remaining_balance >= MIN_REDBAG_SHARE,
            "Sorry, the redbag has few value to be claimed."
        );
        // 判断用户是否领取过
        assert!(
            rb.claim_info
                .iter()
                .filter(|x| x.user == account_id)
                .count()
                == 0,
            "Sorry, you have claimed this redbag before."
        );
        // 领取红包 如果是最后一个领取人，则拿走所有
        let amount = if rb.claim_info.len() == rb.count as usize - 1 {
            rb.remaining_balance
        } else {
            if rb.mode == 1 {
                self.random_amount(rb.remaining_balance)
            } else {
                self.even_amount(rb.balance.into(), rb.count)
            }
        };
        // 更新红包记录
        rb.remaining_balance -= amount;
        let ci = ClaimInfo {
            user: account_id.clone(),
            amount,
            height: env::block_index(),
            ts: env::block_timestamp(),
        };
        rb.claim_info.push(ci);
        self.red_info.insert(&pk, &rb);

        // 更新领取人记录
        let mut receiver_record = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        receiver_record.push(pk.clone());
        self.receiver_redbag.insert(&account_id, &receiver_record);
        amount
    }

    /// Redbag receiver may encounter errors that 
    /// may cause the create_account_and_claim fail, 
    /// in this case, the claim_info vec in red_info 
    /// and receiver_redbag map need to be rolled back.
    fn remove_recv(&mut self, pk: PublicKey, account_id: AccountId, 
        height: u64, amount: Balance) -> bool {

        // handle roll back of red_info
        let redbag = self.red_info.get(&pk);
        if redbag.is_none() {
            return false;
        }
        let mut rb = &mut redbag.unwrap();
        rb.remaining_balance += amount;
        let new_claim_info: Vec<ClaimInfo> = rb.claim_info
            .iter().filter(|x| x.user != account_id 
                || x.height != height 
                || x.amount != amount)
            .map(|x| x.clone()).collect();
        rb.claim_info = new_claim_info;
        self.red_info.insert(&pk, &rb);
        
        // handle roll back of receiver_redbag
        let receiver_record = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        let new_recv_record: Vec<PublicKey> = receiver_record
            .iter().filter(|x| **x != pk).map(|x| x.clone()).collect();
        if receiver_record.len() > 0 {
            self.receiver_redbag.insert(&account_id, &new_recv_record);
        } else {
            self.receiver_redbag.remove(&account_id);
        }
        
        true
    }

    /// 生成随机, 255个层级 total_amount * share_rate / u8::max_value().into()
    fn random_amount(&self, total_amount: u128) -> u128 {
        // todo:
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

        let random_share = min_share.wrapping_mul(share_rate.into());
        if random_share >= MIN_REDBAG_SHARE {
            random_share
        } else {
            MIN_REDBAG_SHARE
        }
    }

    fn even_amount(&self, total_amount: u128, count: u8) -> u128 {
        let even_share = total_amount / count as u128;
        if even_share >= MIN_REDBAG_SHARE {
            even_share
        } else {
            MIN_REDBAG_SHARE
        }
    }

    fn format_claiminfo(&self, claiminfo: &ClaimInfo) -> HumanReadableClaimInfo {
        HumanReadableClaimInfo {
            user: claiminfo.user.clone(),
            amount: claiminfo.amount.into(),
            height: claiminfo.height.into(),
            ts: claiminfo.ts.into(),
        }
    }

    fn redbag_brief(&self, pk: &PublicKey) -> HumanReadableRedBrief {
        let redbag_info = self.red_info.get(pk).unwrap();
        HumanReadableRedBrief {
            owner: redbag_info.owner,
            id: (*pk).clone().try_into().unwrap(),
            mode: redbag_info.mode,
            count: redbag_info.count,
            balance: redbag_info.balance.into(),
            remaining_balance: redbag_info.remaining_balance.into(),
            received_count: redbag_info.claim_info.len() as u8,
            height: redbag_info.height.into(),
            ts: redbag_info.ts.into(),
        }
    }

    fn recv_brief(&self, pk: &PublicKey, claim: &ClaimInfo) -> HumanReadableRecvBrief {
        HumanReadableRecvBrief {
            id: pk.clone().try_into().unwrap(),
            balance: claim.amount.into(),
            height: claim.height.into(),
            ts: claim.ts.into(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, BlockHeight, PublicKey, VMContext};
}
