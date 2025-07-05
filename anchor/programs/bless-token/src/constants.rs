use anchor_lang::prelude::*;

pub const NODE_ID_LEN: usize = 36;

pub const SIGN_IX_HEAD_LEN: usize = 16 + 64 + 32;

/// backend sign length
pub const BE_SIGN_LEN: usize = 64;

#[constant]
pub const SEED_ADMIN_RECHARGE: &str = "bless_admin_recharge";

#[constant]
pub const SEED_BLESS_TOKEN_STATE: &str = "bless_token_state";

#[constant]
pub const SEED_TIME_TOKEN_VAULT: &str = "bless_time_token_vault";
