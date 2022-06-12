use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight, Gas, Promise, PromiseResult,
    PublicKey, log,
};
use std::collections::HashSet;
use uint::construct_uint;

mod internal;

/// this 0.5 Near acts as base AKA, which is guaranteed by this contract itself
const BASE_ACCESS_KEY_ALLOWANCE: Balance = 500_000_000_000_000_000_000_000;

/// the total cost that create_account_and_claim is a little less than 0.020 Near
/// to be more secure, we set single_claim_cost to 0.050 Near
/// 
/// the total cost that create_account_and_claim is a little less than 30T gas,
/// equal to 0.003 Near， Now we set to 0.01 Near for secure
const MIN_SINGLE_CLAIM_COST: Balance = 3_000_000_000_000_000_000_000;
const SINGLE_CLAIM_COST: Balance = 10_000_000_000_000_000_000_000;

/// the minimum balance that an account must contain to maintain state fee.
/// 0.1 NEAR
const MIN_REDBAG_SHARE: Balance = 100_000_000_000_000_000_000_000;

/// 20T Gas attached to the callback from account creation.
pub const TGAS: u64 = 1_000_000_000_000;
pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: Gas = Gas(20 * TGAS);

/// Indicates there are no deposit for a callback for better readability.
const NO_DEPOSIT: Balance = 0;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}


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
    pub id: PublicKey,
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
    pub id: PublicKey,
    pub balance: U128,
    pub height: U64,
    pub ts: U64,
}

// HumanReadableStatistic
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableStatistic {
    pub owner_id: AccountId,
    pub base_aka: U128,
    pub single_recv_fee: U128,
    pub total_send_count: u32,  // count of sent redbag
    pub total_recv_count: u32,  // count of redbag receiver
    pub total_revoke_count: u32,  // count of revoke 
    pub total_send_amount: U128,  // total NEAR that sent
    pub total_recv_amount: U128,  // total NEAR that received
    pub total_revoke_amount: U128,  // total NEAR that rovoked
    pub total_account_created: u32,  // count of new account created
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RedBag {
    pub owner_id: AccountId,
    pub base_aka: Balance,
    pub single_recv_fee: Balance,
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

impl Default for RedBag {
    fn default() -> Self {
        env::panic_str("This contract should be initialized before usage")
    }
}

#[near_bindgen]
impl RedBag {

    #[init]
    pub fn new(owner_id: AccountId, ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid");
        
        Self {
            owner_id,
            base_aka: BASE_ACCESS_KEY_ALLOWANCE,
            single_recv_fee: SINGLE_CLAIM_COST,
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

    //***********************/
    // owner functions
    //***********************/

    pub fn change_owner_id(&mut self, new_owner_id: AccountId) {
        self.assert_owner();
        assert!(env::is_valid_account_id(new_owner_id.as_bytes()),
            "The owner account ID is invalid");
        self.owner_id = new_owner_id;
    }

    pub fn update_single_recv_fee(&mut self, single_recv_fee: U128) {
        self.assert_owner();
        let single_recv_fee: Balance = single_recv_fee.into();
        assert!(single_recv_fee > MIN_SINGLE_CLAIM_COST, 
            "single_recv_fee is too low");
        self.single_recv_fee = single_recv_fee;
    }

    pub fn update_base_aka(&mut self, base_aka: U128) {
        self.assert_owner();
        self.base_aka = base_aka.into();
    }

    //***********************/
    // user functions
    //***********************/

    /// 发红包功能
    #[payable]
    pub fn send_redbag(
        &mut self,
        public_key: PublicKey,
        count: u8,
        mode: u8,
        slogan: String,
    ) -> Promise {
        let fee_cost: Balance = count as Balance * self.single_recv_fee;
        assert!(
            env::attached_deposit() > fee_cost + count as Balance * MIN_REDBAG_SHARE,
            "Attached deposit must be greater than count * (single_recv_fee + MIN_REDBAG_SHARE)"
        );

        let owner = env::signer_account_id();
        let pk: PublicKey = public_key.clone();
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
            balance: env::attached_deposit() - fee_cost,
            remaining_balance: env::attached_deposit() - fee_cost,
            height: env::block_height(),
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
        self.total_send_amount += env::attached_deposit() - fee_cost;

        // 添加 access key
        Promise::new(env::current_account_id()).add_access_key(
            pk,
            self.base_aka + fee_cost,
            env::current_account_id(),
            "create_account_and_claim,claim".to_string(),
        )
    }

    /// 创建新用户同时领取红包
    pub fn create_account_and_claim(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> Promise {
        let pk = env::signer_account_pk();
        let amount = self.claim_redbag(pk.clone(), new_account_id.clone());

        Promise::new(new_account_id.clone())
            .create_account()
            .add_full_access_key(new_public_key)
            .transfer(amount)
            .then(ext_self::on_account_created_and_claimed_ex(
                new_account_id,
                env::block_height().into(),
                amount.into(),
                env::current_account_id(),
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
    pub fn revoke(&mut self, public_key: PublicKey) -> Promise {
        let pk: PublicKey = public_key.clone();
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
            height: env::block_height(),
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
    pub fn get_key_balance(&self, key: PublicKey) -> U128 {
        self.red_info.get(&key).expect("Key is missing")
            .remaining_balance
            .into()
    }

    pub fn show_statistic(&self) -> HumanReadableStatistic {
        HumanReadableStatistic {
            owner_id: self.owner_id.clone(),
            base_aka: self.base_aka.into(),
            single_recv_fee: self.single_recv_fee.into(),
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
    pub fn show_redbag_detail(&self, public_key: PublicKey) ->  HumanReadableRedDetail {
        // let pk = public_key.into();
        // 查看红包是否存在
        let redbag = self.red_info.get(&public_key);
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
    pub fn show_redbag_brief(&self, public_key: PublicKey) -> HumanReadableRedBrief {
        // let pk = public_key.into();
        self.redbag_brief(&public_key)
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
            log!(
                format!(
                    "Create account and claim failed! Redbag info rolled back: {}",
                    remove_ret
                )
            );
            
        }
        creation_ret
    }
}
