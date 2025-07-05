#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
mod constants;
mod context;
mod errors;
mod states;
pub use constants::*;
pub use context::*;
pub use states::*;

declare_id!("93HzBKdD4w8jfBBdnbjdDs9NeiJB6xHfkrSTmVxLTiQD");

#[program]
pub mod bless_time {
    use super::*;

    /// register function
    pub fn initial(ctx: Context<InitBlessToken>) -> Result<()> {
        Ok(())
    }
}
