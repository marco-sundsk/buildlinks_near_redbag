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

/// the total cost that create_account_and_claim is a little less than 0.020 Near
/// to be more secure, we set single_claim_cost to 0.050 Near
const SINGLE_CLAIM_COST: Balance = 50_000_000_000_000_000_000_000;

/// the minimum balance that an account must contain to maintain state fee.
/// 0.1 NEAR
const MIN_REDBAG_SHARE: Balance = 100_000_000_000_000_000_000_000;

/// 20T Gas attached to the callback from account creation.
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
    pub slogan: String,
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

// HumanReadableStatistic
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableStatistic {
    pub total_send_count: u32,  // count of sent redbag
    pub total_recv_count: u32,  // count of redbag receiver
    pub total_revoke_count: u32,  // count of revoke 
    pub total_send_amount: U128,  // total NEAR that sent
    pub total_recv_amount: U128,  // total NEAR that received
    pub total_revoke_amount: U128,  // total NEAR that rovoked
    pub total_account_created: u32,  // count of new account created
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
    // total redbag send info
    pub total_send_count: u32,
    pub total_recv_count: u32,
    pub total_revoke_count: u32,
    // toal redbag recv info
    pub total_send_amount: Balance,
    pub total_recv_amount: Balance,
    pub total_revoke_amount: Balance,
    // total new accounts created
    pub total_account_created: u32,
}

#[ext_contract(ext_self)]
pub trait ExtRedBag {
    /// Callback after creating account and claiming redbag.
    fn on_account_created_and_claimed_ex(&mut self, 
        account_id: AccountId, height: U64, amount: U128) -> bool;
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
            total_send_count: 0,
            total_recv_count: 0,
            total_revoke_count: 0,
            total_send_amount: 0,
            total_recv_amount: 0,
            total_revoke_amount: 0,
            total_account_created: 0,
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
        let ak_allowance: Balance = count as Balance * SINGLE_CLAIM_COST;
        assert!(
            env::attached_deposit() > ak_allowance + count as Balance * MIN_REDBAG_SHARE,
            "Attached deposit must be greater than count * (SINGLE_CLAIM_COST + MIN_REDBAG_SHARE)"
        );

        let owner = env::signer_account_id();
        let pk: PublicKey = public_key.clone().into();
        assert!(
            self.red_info.get(&pk).is_none(),
            "The public_key already exists"
        );

        // make sure slogan would NOT be too long
        let short_slogan: String;
        let slogan_vec = slogan.chars().collect::<Vec<_>>();
        if slogan_vec.len() > 32 {
            short_slogan = slogan_vec[..32].iter().cloned().collect::<String>();
        } else {
            short_slogan = slogan_vec[..].iter().cloned().collect::<String>();
        }

        // 初始化红包信息并记录
        let new_red_info = RedInfo {
            owner: owner.clone(),
            mode,
            count,
            slogan: short_slogan.to_string(),
            balance: env::attached_deposit() - ak_allowance,
            remaining_balance: env::attached_deposit() - ak_allowance,
            height: env::block_index(),
            ts: env::block_timestamp(),
            claim_info: Vec::new(),
        };
        self.red_info.insert(&pk, &new_red_info);
        // 更新账户的发红包记录
        let mut relation_vec = self.sender_redbag.get(&owner).unwrap_or(Vec::new());
        relation_vec.push(pk.clone());
        self.sender_redbag.insert(&owner, &relation_vec);
        // 更新统计信息
        self.total_send_count += 1;
        self.total_send_amount += env::attached_deposit() - ak_allowance;

        // 添加 access key
        Promise::new(env::current_account_id()).add_access_key(
            pk,
            ak_allowance,
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

        Promise::new(new_account_id.clone())
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(amount)
            .then(ext_self::on_account_created_and_claimed_ex(
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

        if self.is_redbag_closed(pk) {
            Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
        }

        self.total_recv_count += 1;
        self.total_recv_amount += amount;
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

        Promise::new(env::current_account_id()).delete_key(pk);

        self.total_revoke_count += 1;
        self.total_revoke_amount += amount;
        Promise::new(account_id).transfer(amount)
    }

    /************************/
    /* view functions       */
    /************************/

    /// Returns the balance associated with given key.
    pub fn get_key_balance(&self, key: Base58PublicKey) -> U128 {
        self.red_info.get(&key.into()).expect("Key is missing")
            .remaining_balance
            .into()
    }

    pub fn show_statistic(&self) -> HumanReadableStatistic {
        HumanReadableStatistic {
            total_send_count: self.total_send_count,
            total_recv_count: self.total_recv_count,
            total_revoke_count: self.total_revoke_count,
            total_send_amount: self.total_send_amount.into(),
            total_recv_amount: self.total_recv_amount.into(),
            total_revoke_amount: self.total_revoke_amount.into(),
            total_account_created: self.total_account_created,
        }
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
            slogan: redbag_info.slogan.clone(),
            mode: redbag_info.mode,
            count: redbag_info.count,
            balance: redbag_info.balance.into(),
            remaining_balance: redbag_info.remaining_balance.into(),
            height: redbag_info.height.into(),
            ts: redbag_info.ts.into(),
            claim_info: cis,
        }
    }

    /// 查询用户所发的所有红包
    pub fn show_send_list(&self, account_id: AccountId) -> Vec<HumanReadableRedBrief> {
        let relation_vec = self.sender_redbag.get(&account_id).unwrap_or(Vec::new());
        relation_vec
            .iter()
            .map(|x| self.redbag_brief(x))
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
    pub fn on_account_created_and_claimed_ex(&mut self, 
        account_id: AccountId, height: U64, amount: U128) -> bool {
            assert_eq!(
                env::predecessor_account_id(),
                env::current_account_id(),
                "Callback can only be called from the contract"
            );
            let amount: Balance = amount.into();
            let creation_ret = is_promise_success();
            if creation_ret {
                if self.is_redbag_closed(env::signer_account_pk()) {
                    Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
                }
                self.total_account_created += 1;
                self.total_recv_count += 1;
                self.total_recv_amount += amount;
            } else {
                // In case of failure, put the amount back.
                // 失败的情况下，回退资金及相关结构信息的更改
                let remove_ret = self.remove_recv(env::signer_account_pk(), account_id, height.into(), amount);
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

        // 判断用户是否领取过
        assert!(
            rb.claim_info
                .iter()
                .filter(|x| x.user == account_id)
                .count()
                == 0,
            "Sorry, you have claimed this redbag before."
        );
        
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
        
        // 领取红包 如果是最后一个领取人，则拿走所有
        let amount = if rb.claim_info.len() == rb.count as usize - 1 {
            rb.remaining_balance
        } else {
            if rb.mode == 1 {
                self.random_amount(&rb)
            } else {
                self.even_amount(&rb)
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

    fn is_redbag_closed(&self, pk: PublicKey) -> bool {
        let redbag = self.red_info.get(&pk).expect("No redbag found for this pk!");

        if redbag.remaining_balance < MIN_REDBAG_SHARE {
            return true;
        }

        if redbag.claim_info.len() == redbag.count as usize {
            return true;
        }

        false
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
    fn random_amount(&self, rb: &RedInfo) -> u128 {

        // figure out standard share
        // then got random scope is [standard/2, standard*3/2]
        let standard = rb.remaining_balance / (rb.count as usize - rb.claim_info.len()) as u128;
        let left_bound = standard / 2;
        let right_bound = standard * 3 / 2;

        // got random in scope
        let random_u8: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
        let offset = (right_bound - left_bound) * random_u8 as u128 / 0x100_u128;

        env::log(
            format!(
                "Create random {} in scope [{}, {}]",
                offset, left_bound, right_bound
            )
            .as_bytes(),
        );

        // keep random_share illegal 
        let random_share = left_bound + offset;
        if random_share >= MIN_REDBAG_SHARE {
            random_share
        } else {
            MIN_REDBAG_SHARE
        }
    }

    fn even_amount(&self, rb: &RedInfo) -> u128 {
        let even_share = rb.balance / rb.count as u128;
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
