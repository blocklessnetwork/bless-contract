#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
mod constants;
mod context;
mod errors;
mod states;
pub use constants::*;
pub use context::*;
pub use states::*;

declare_id!("6QtrRhkvR7YXAvbMqf3gEH29etrFZw1g1MrCVxQ2Muvq");

#[program]
pub mod bless_token {

    use super::*;

    /// initial bless token state function
    pub fn bless_token_initial(ctx: Context<InitBlessToken>) -> Result<()> {
        let bump = ctx.bumps.bless_state;
        let vault_bump = ctx.bumps.vault_account;
        ctx.accounts.init(bump, vault_bump)
    }

    /// distribute to packet account period.
    pub fn fund_bless_token(ctx: Context<FundBlessToken>) -> Result<()> {
        ctx.accounts.fund()
    }
}
