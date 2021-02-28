# BuildlinksRedbag4Near contract

BuildlinksRedbag4Near contract allows any user to create a redbag that their online group friends can grab to claim tokens even if they don't have an account yet.



Play with this contract
========================
the contract is deployed at testnet with the name `rb01.testnet`

you can set it to env for later use:
```shell
export CONTRACTID=rb01.testnet
```

## Look around
```shell
# 
near view $CONTRACTID show_redbag_brief '{"public_key": "xxxxxxxxxxx"}'
# 
near view $CONTRACTID show_redbag_detail '{"public_key": "xxxxxxxxxxx"}'
# 
near view $CONTRACTID show_send '{"account_id": "humeng.testnet"}'
near view $CONTRACTID show_send_list '{"account_id": "humeng.testnet"}'
# 
near view $CONTRACTID show_recv '{"account_id": "xxxxxxxxxxx"}'
near view $CONTRACTID show_recv_list '{"account_id": "xxxxxx"}'
```
## Let's play
```shell
# attached 5 Near
near call $CONTRACTID send_redbag '{"public_key": "xxxx", "count": 2, "mode": 1, "slogan": "aaa"}' --amount=5 --account_id=humeng.testnet
# 
near call $CONTRACTID create_account_and_claim '{"new_account_id": "", "new_public_key": ""}' --account_id=alice.testnet

near call $CONTRACTID claim '{"account_id": ""}' --account_id=alice.testnet

near call $CONTRACTID revoke '{"public_key": "aaaaaaa"}' --account_id=alice.testnet
```

Build Deploy and Init
======================

Before you compile this code, you will need to install Rust with [correct target]


```shell
# building it
srouce ./build.sh
```

```shell
# deploy it
near deploy rb01.testnet res/redbag2.wasm --account_id=rb01.testnet

# say it was deploy at $CONTRACTID, then init it 
near call $CONTRACTID new \
  '{"owner_id": "boss.testnet", "dice_number": 1, 
  "rolling_fee": "1000000000000000000000000", 
  "reward_fee_fraction": {"numerator": 5, "denominator": 100}}' \
  --account_id=$CONTRACTID
```


Contract Interface
==================

### Data Structure
```rust
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
```

### Send Claim and Revoke Redbag
```rust
/// 发红包功能
#[payable]
pub fn send_redbag(&mut self, public_key: Base58PublicKey,
    count: u8, mode: u8, slogan: String,) -> Promise;

/// 创建新用户同时领取红包
pub fn create_account_and_claim(&mut self, new_account_id: AccountId,
    new_public_key: Base58PublicKey,) -> Promise;

/// 领取红包
pub fn claim(&mut self, account_id: AccountId) -> Promise;

/// 红包所有人撤回对应public_key的红包剩余金额
/// 撤回视为自己领取剩余金额
pub fn revoke(&mut self, public_key: Base58PublicKey) -> Promise;
```

### View interfaces
```rust
pub fn show_send_list(&self, account_id: AccountId) -> Vec<HumanReadableRedBrief>;

pub fn show_recv_list(&self, account_id: AccountId) -> Vec<HumanReadableRecvBrief>;

pub fn show_redbag_brief(&self, public_key: Base58PublicKey) -> HumanReadableRedBrief;

pub fn show_redbag_detail(&self, public_key: Base58PublicKey) ->  HumanReadableRedDetail;
```


The way it works:

Sender, that has NEAR:
- Creates a new key pair `(pk1, privkey1)`.
- Calls `redbag.send_redbag(pk1, count, mode, slogan)` with attached balance of NEAR that they want to wrap into the redbag.
- Sends a redbag link to any online group with `privkey1` behind the link.

note: `count` indicates the number of receivers that can share the redbag; `mode` decides a random share with value 1 or an even share with value 0; `slogan` is a blessing words that would be the caption of that redbag.

Receiver, that doesn't have NEAR:
- Receives link to the wallet with `privkey1`.
- Wallet creates new key pair for this user (or they generate it via HSM) `(pk2, privkey2)`.
- Enters the `new_account_id` receiver want for their new account.
- Wallet creates a transaction to `redbag.create_account_and_claim(new_account_id, pk2)`.
- Contract creates new account with `new_account_id` name and `pk2` as full access key and transfers his share of the redbag.

If Receiver already has account:
- Sign tx with `privkey1` to call `redbag.claim()`, which transfers his share of the redbag to signer's account.

If Sender wants to revoke the redbag):
- Sign tx with `privkey1` to call `redbag.revoke()`, which transfers remaining money back to sender's account.

