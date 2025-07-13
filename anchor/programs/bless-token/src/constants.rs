use anchor_lang::prelude::*;

#[constant]
pub const SEED_BLESS_CONTRACT_STATE: &str = "bless_contract_state";

#[constant]
pub const SEED_BLESS_VAULT: &str = "bless_vault_account";

pub const BITMAP_SIZE: usize = 86;

pub const MAX_MONTHS: usize = BITMAP_SIZE;

pub const PRE_SEED_SALE_FUND_FEE: u64 = 78_125_000;

pub const SEED_SALE_FUND_FEE: u64 = 125_000_000;

pub const ADVISORS_FUND_FEE: u64 = 15_000_000;

pub const TEAM_FUND_FEE: u64 = 75_000_000;

pub const ECOSYSTEM_FUND_FEE: u64 = 34_375_000;

pub const FOUNDATION_FEE: u64 = 100_000_000;

pub const MARKET_MAKING_FUND_FEE: u64 = 300_000_000;

pub const TGE_MARKETING_FUND_FEE: u64 = 60_000_000;

pub const COMMUNITY_AIRDROP_FUND_FEE: u64 = 850_000_000;

pub const COMMUNITY_INCENTIVES_FUN_FEE: u64 = 3_500_000_000;

pub const WALLET_INVESTOR_FEE: u64 = 1625000000;

pub const WALLET_TEAM_ADVISOR_FEE: u64 = 1800000000;

pub const WALLET_FOUNDATION_FEE: u64 = 1000000000;

pub const WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE: u64 = 1075000000;

pub const WALLET_COMMUNITY_REWARDS_FEE: u64 = 4500000000;
