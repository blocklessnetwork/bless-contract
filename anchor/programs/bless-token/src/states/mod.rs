use anchor_lang::prelude::*;

use crate::BITMAP_SIZE;

mod bitvec;
pub use bitvec::BitVec;

/// RuleOutcome is the fund rule and the fund result.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, InitSpace)]
pub struct RuleOutcome {
    // the fund rule per month.
    pub rule: BitVec,
    // the fund result per month.
    pub outcome: BitVec,
}

impl Default for RuleOutcome {
    fn default() -> Self {
        Self {
            // Each account follows a funding schedule spanning 86 months.
            rule: BitVec::new(BITMAP_SIZE),
            // Each account tracks funding execution results for 86 months.
            outcome: BitVec::new(BITMAP_SIZE),
        }
    }
}

/// BlessTokenState is store the bless mint and the packet token accounts.
/// mint_pubkey is the public key of $BLESS token mint.
#[account]
#[derive(InitSpace)]
pub struct BlessTokenState {
    // the vault token public key.
    pub mint_pubkey: Pubkey,

    // gets 16.25%
    pub wallet_investor: Pubkey,

    //gets 18%
    pub wallet_team_advisor: Pubkey,

    //gets 10%
    pub wallet_foundation: Pubkey,

    //get 10.75%
    pub wallet_ecosystem_liquidityprovision_tgtmarketing: Pubkey,

    //gets 45%
    pub wallet_community_rewards: Pubkey,

    pub preseed_sale: Pubkey,

    pub preseed_sale_rule_outcome: RuleOutcome,

    pub seed_sale: Pubkey,

    pub seed_sale_rule_outcome: RuleOutcome,

    pub advisors: Pubkey,

    pub advisors_rule_outcome: RuleOutcome,

    pub team: Pubkey,

    pub team_rule_outcome: RuleOutcome,

    pub ecosystem: Pubkey,

    pub ecosystem_rule_outcome: RuleOutcome,

    pub foundation: Pubkey,

    pub foundation_rule_outcome: RuleOutcome,

    pub liquidity_provision: Pubkey,

    pub liquidity_provision_rule_outcome: RuleOutcome,

    pub tgt_marketing: Pubkey,

    pub tgt_marketing_rule_outcome: RuleOutcome,

    pub airdrop: Pubkey,

    pub airdrop_rule_outcome: RuleOutcome,

    pub community_rewards: Pubkey,

    pub community_rewards_rule_outcome: RuleOutcome,

    pub current_month: u8,

    pub bump: u8,
}
