use crate::*;

impl RedBagContract {
    
    ///
    pub(crate) fn claim_redbag(&mut self, pk: PublicKey, account_id: AccountId) -> Balance {
        // 查看红包是否存在
        let redbag = self.red_info.get(&pk);
        assert!(redbag.is_some(), "No corresponding redbag found.");

        // 查看红包剩余数量是否可被领取
        let mut rb = &mut redbag.unwrap();
        assert!(rb.claim_info.len() < rb.count as usize, 
            "Sorry, the redbag has been claimed out.");
        assert!(rb.remaining_balance != 0, 
            "Sorry, the redbag has been revoked.");
        assert!(rb.remaining_balance >= MIN_REDBAG_SHARE, 
            "Sorry, the redbag has few value to be claimed.");
        // 判断用户是否领取过
        assert!(rb.claim_info.iter().filter(|x| x.user == account_id).count() == 0, 
            "Sorry, you have claimed this redbag before.");
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
        };
        rb.claim_info.push(ci);
        self.red_info.insert(&pk, &rb);
        // 更新领取人记录
        let mut receiver_record = self.receiver_redbag.get(&account_id).unwrap_or(Vec::new());
        receiver_record.push(pk.clone());
        self.receiver_redbag.insert(&account_id, &receiver_record);
        
        amount
    }

    /// 生成随机, 255个层级 total_amount * share_rate / u8::max_value().into()
    pub(crate) fn random_amount(&self, total_amount: u128) -> u128 {
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

    /// 均分金额
    pub(crate) fn even_amount(&self, total_amount: u128, count: u128) -> u128 {
        let even_share = total_amount / count;
        if even_share >= MIN_REDBAG_SHARE {
            even_share
        } else {
            MIN_REDBAG_SHARE
        }
    }
}