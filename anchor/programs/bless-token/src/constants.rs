use anchor_lang::prelude::*;

#[constant]
pub const SEED_BLESS_CONTRACT_STATE: &str = "bless_contract_state";

pub const BITMAP_SIZE: usize = 86;

pub const WALLET_INVESTOR_FEE: u64 = 1625000000;

pub const WALLET_TEAM_ADVISOR_FEE: u64 = 1800000000;

pub const WALLET_FOUNDATION_FEE: u64 = 1000000000;

pub const WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE: u64 = 1075000000;

pub const WALLET_COMMUNITY_REWARDS_FEE: u64 = 4500000000;

#[cfg(feature = "testnet")]
pub const MINT_KEY: &str = "7jXWwbXguMzhxcGVGUd7jPahqJ33eAVke5Kav13RZNAe";

// todo replease use bless token address.
#[cfg(not(feature = "testnet"))]
pub const MINT_KEY: &str = "7jXWwbXguMzhxcGVGUd7jPahqJ33eAVke5Kav13RZNAe";
