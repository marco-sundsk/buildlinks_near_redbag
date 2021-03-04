use crate::*;

impl RedBag {
    /********************/
    /* Internal methods */
    /********************/
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }

    pub(crate) fn claim_redbag(&mut self, pk: PublicKey, account_id: AccountId) -> Balance {
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

    pub(crate) fn is_redbag_closed(&self, pk: PublicKey) -> bool {
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
    pub(crate) fn remove_recv(&mut self, pk: PublicKey, account_id: AccountId, 
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
    pub(crate) fn random_amount(&self, rb: &RedInfo) -> u128 {

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

    pub(crate) fn even_amount(&self, rb: &RedInfo) -> u128 {
        let even_share = rb.balance / rb.count as u128;
        if even_share >= MIN_REDBAG_SHARE {
            even_share
        } else {
            MIN_REDBAG_SHARE
        }
    }

    pub(crate) fn format_claiminfo(&self, claiminfo: &ClaimInfo) -> HumanReadableClaimInfo {
        HumanReadableClaimInfo {
            user: claiminfo.user.clone(),
            amount: claiminfo.amount.into(),
            height: claiminfo.height.into(),
            ts: claiminfo.ts.into(),
        }
    }

    pub(crate) fn redbag_brief(&self, pk: &PublicKey) -> HumanReadableRedBrief {
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

    pub(crate) fn recv_brief(&self, pk: &PublicKey, claim: &ClaimInfo) -> HumanReadableRecvBrief {
        HumanReadableRecvBrief {
            id: pk.clone().try_into().unwrap(),
            balance: claim.amount.into(),
            height: claim.height.into(),
            ts: claim.ts.into(),
        }
    }
}