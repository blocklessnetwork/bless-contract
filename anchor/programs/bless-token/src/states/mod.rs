use anchor_lang::prelude::*;
mod bitvec;

/// BlessTokenState is store the bless mint and the packet token accounts.
/// mint_pubkey is the public key of $BLESS token mint.
#[account]
#[derive(InitSpace)]
pub struct BlessTokenState {
    // the vault token public key.
    pub mint_pubkey: Pubkey,

    // gets 16.25%
    pub wallet_investor: Pubkey,
    pub wallet_investor_token: u64,

    //gets 18%
    pub wallet_team_advisor: Pubkey,
    pub wallet_team_advisor_token: u64,

    //gets 10%
    pub wallet_foundation: Pubkey,
    pub wallet_foundation_token: u64,

    //get 10.75%
    pub wallet_ecosystem_liquidityprovision_tgtmarketing: Pubkey,
    pub wallet_ecosystem_liquidityprovision_tgtmarketing_token: u64,

    //gets 45%
    pub wallet_community_rewards: Pubkey,
    pub wallet_community_rewards_token: u64,

    pub bump: u8,
}
