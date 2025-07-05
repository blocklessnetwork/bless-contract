use anchor_lang::prelude::*;

use crate::states::bitvec::BitVec;

mod bitvec;

/// Vault account store $TIME or $BLESS token recharge by admin through the time points.
/// mint_pubkey is the public key of $TIME token or $BLESS token.
/// amount is the amount vault account.
/// time is call recharge time on chain.
#[account]
#[derive(InitSpace)]
pub struct BlessTokenState {
    // the vault token public key.
    pub mint_pubkey: Pubkey,

    pub preseed_sale: Pubkey,

    pub preseed_sale_rule: BitVec,

    pub seed_sale: Pubkey,

    pub advisors: Pubkey,

    pub team: Pubkey,

    pub ecosystem: Pubkey,

    pub foundation: Pubkey,

    pub liquidity_provision: Pubkey,

    pub tgt_marketing: Pubkey,

    pub airdrop: Pubkey,

    pub community_rewards: Pubkey,

    pub bump: u8,
}
