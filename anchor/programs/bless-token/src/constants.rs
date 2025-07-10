use anchor_lang::prelude::*;

#[constant]
pub const SEED_BLESS_CONTRACT_STATE: &str = "bless_contract_state";

#[constant]
pub const SEED_BLESS_VAULT: &str = "bless_vault_account";

pub const BITMAP_SIZE: usize = 86;

pub const PRE_SEED_SALE_FUND_FEE: u64 = 78_125_000;

pub const SEED_SALE_FUND_FEE: u64 = 125_000_000;

pub const ADVISORS_FUND_FEE: u64 = 15_000_000;

pub const TEAM_FUND_FEE: u64 = 75_000_000;

pub const ECOSYSTEM_GENESIS_FUND_FEE: u64 = 300_000_000;
pub const ECOSYSTEM_FUND_FEE: u64 = 34_375_000;

pub const FOUNDATION_FUND_FEE: u64 = 100_000_000;

pub const LIQUIDITY_PROVISION_FUND_FEE: u64 = 300_000_000;

pub const TGE_MARKETING_FUND_FEE: u64 = 200_000_000;

pub const AIRDROP_FUND_FEE: u64 = 1_000_000_000;

pub const COMMUNITY_REWARDS_FEE: u64 = 46_000_000;
// The funding fee will be reduced by 1 million each month for 5 months.
pub const COMMUNITY_REWARDS_FEE_REDUCE_TIMES: u8 = 5;
pub const COMMUNITY_REWARDS_FEE_REDUCE_FEE: u64 = 1_000_000;

pub const WALLET_INVESTOR_FEE: u64 = 1625000000;

pub const WALLET_TEAM_ADVISOR_FEE: u64 = 1800000000;

pub const WALLET_FOUNDATION_FEE: u64 = 1000000000;

pub const WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE: u64 = 1075000000;

pub const WALLET_COMMUNITY_REWARDS_FEE: u64 = 4500000000;
