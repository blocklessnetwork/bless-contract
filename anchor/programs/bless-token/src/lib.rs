#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
mod constants;
mod context;
mod errors;
mod states;
pub use constants::*;
pub use context::*;
pub use states::*;

declare_id!("uqmCn7C32qa1MvbiC6MxLnnVVLFnutx3tJ7sdDT73g2");

#[program]
pub mod bless_token {

    use super::*;

    /// initial bless token state function
    pub fn initialize_bless_token(ctx: Context<InitBlessToken>) -> Result<()> {
        let bump = ctx.bumps.bless_state;
        ctx.accounts.init(bump)
    }
}
