#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
mod constants;
mod context;
mod errors;
mod states;
pub use constants::*;
pub use context::*;
pub use states::*;

declare_id!("12P2dRDRBw5anmVTPmJELjTCgFeAMmbHM96SXsMZzKkK");

#[program]
pub mod bless_token {

    use super::*;

    /// initial bless token state function
    pub fn initialize_bless_token(ctx: Context<InitBlessToken>) -> Result<()> {
        let bump = ctx.bumps.bless_state;
        ctx.accounts.init(bump)
    }

    /// Initializes the bless meta state with the payer designated as the admin.
    /// This function can only be called once.
    pub fn initialize_bless_token_meta_state(
        ctx: Context<InitialBlessTokenMetaState>,
    ) -> Result<()> {
        let bump = ctx.bumps.bless_meta_state;
        ctx.accounts.initial(bump)
    }

    /// This instruction should take the new administrator's public key as an argument.
    /// It must be callable only by the current admin.
    /// Its logic should update the pending_admin field with the new administrator's address.
    pub fn set_pending_admin_account(ctx: Context<BlessTokenMetaProposeAdmin>) -> Result<()> {
        ctx.accounts.propose_admin()
    }

    /// This instruction must be callable only by the key stored in pending_admin.
    /// Its logic should update the admin to the pending_admin's address
    /// and then clear the pending_admin to finalize the transfer.
    pub fn accept_admin(ctx: Context<BlessTokenMetaAcceptAdmin>) -> Result<()> {
        ctx.accounts.accept_admin()
    }

    /// Creates token metadata with name, symbol, and metadata URI.
    /// This is usually called during initialization to bind the token
    /// to the Metaplex Token Metadata program.
    pub fn create_metadata(
        ctx: Context<BlessTokenMetaSetMetadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create_metadata(name, symbol, uri)
    }

    /// Update token metadata with name, symbol, and metadata URI.
    /// Unlike `create_metadata`, this is called after initialization
    /// to modify the existing metadata entry stored by the
    /// Metaplex Token Metadata program.
    pub fn update_metadata(
        ctx: Context<BlessTokenMetaSetMetadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.update_metadata(name, symbol, uri)
    }
}
