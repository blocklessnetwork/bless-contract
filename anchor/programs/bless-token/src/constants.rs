use anchor_lang::prelude::*;

#[constant]
pub const SEED_BLESS_CONTRACT_STATE: &str = "bless_contract_state";

#[constant]
pub const SEED_BLESS_TOKEN_META_STATE: &str = "bless_token_meta_state";

pub const BITMAP_SIZE: usize = 86;

pub const WALLET_INVESTOR_FEE: u64 = 1625_000_000_000_000_000;

pub const WALLET_TEAM_ADVISOR_FEE: u64 = 1800000000_000_000_000;

pub const WALLET_FOUNDATION_FEE: u64 = 1000000000_000_000_000;

pub const WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE: u64 = 1075000000_000_000_000;

pub const WALLET_COMMUNITY_REWARDS_FEE: u64 = 4500000000_000_000_000;

pub const MINT_KEY: &str = "A1t2UviBYwyfYZDJyKY2W6Td8ritgsCriUDuNaAQN49S";
