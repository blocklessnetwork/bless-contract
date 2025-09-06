use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::{
        CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{Creator, DataV2},
};

use crate::{
    errors::BlsError, BlessTokenMetaState, BlessTokenState, SEED_BLESS_CONTRACT_STATE,
    SEED_BLESS_TOKEN_META_STATE,
};

#[derive(Accounts)]
pub struct InitialBlessTokenMetaState<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub bless_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.bump,
    )]
    pub bless_state: Box<Account<'info, BlessTokenState>>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_BLESS_TOKEN_META_STATE.as_bytes(), bless_state.key().as_ref()],
        space = 8 + BlessTokenMetaState::INIT_SPACE,
        bump,
    )]
    pub bless_meta_state: Box<Account<'info, BlessTokenMetaState>>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitialBlessTokenMetaState<'info> {
    // initial the token meta admin account.
    pub fn initial(&mut self, bump: u8) -> Result<()> {
        self.bless_meta_state.set_inner(BlessTokenMetaState {
            bump,
            admin: *self.payer.key,
            pending_admin: Default::default(),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BlessTokenMetaProposeAdmin<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        // only admin call this function
        constraint = admin.key() == bless_meta_state.admin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.bump,
    )]
    pub bless_state: Box<Account<'info, BlessTokenState>>,

    #[account(mut)]
    pub bless_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_BLESS_TOKEN_META_STATE.as_bytes(), bless_state.key().as_ref()],
        bump = bless_meta_state.bump,
    )]
    pub bless_meta_state: Account<'info, BlessTokenMetaState>,

    /// CHECK: this is pending admin account
    pub pending_admin: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> BlessTokenMetaProposeAdmin<'info> {
    pub fn propose_admin(&mut self) -> Result<()> {
        //if the pending admin provided matches the current one,
        // the payer will assume the cost of transfer but no change will be applied
        if self.bless_meta_state.pending_admin == self.pending_admin.key() {
            return Err(BlsError::InvalidPendingAdmin.into());
        }
        //if the pending admin provided matches the current admin,
        // there will be no change when accept_pending_admin but the transfer mechanism will have a cost.
        if self.bless_meta_state.admin == self.pending_admin.key() {
            return Err(BlsError::InvalidPendingAdmin.into());
        }
        self.bless_meta_state.pending_admin = self.pending_admin.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BlessTokenMetaAcceptAdmin<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        // only pending admin call this function
        constraint = pending_admin.key() == bless_meta_state.pending_admin
    )]
    pub pending_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.bump,
    )]
    pub bless_state: Box<Account<'info, BlessTokenState>>,

    #[account(mut)]
    pub bless_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_BLESS_TOKEN_META_STATE.as_bytes(), bless_state.key().as_ref()],
        bump = bless_meta_state.bump,
    )]
    pub bless_meta_state: Account<'info, BlessTokenMetaState>,

    pub system_program: Program<'info, System>,
}

impl<'info> BlessTokenMetaAcceptAdmin<'info> {
    pub fn accept_admin(&mut self) -> Result<()> {
        self.bless_meta_state.admin = self.bless_meta_state.pending_admin;
        // reset
        self.bless_meta_state.pending_admin = Default::default();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BlessTokenMetaSetMetadata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        // only admin call this function
        constraint = admin.key() == bless_meta_state.admin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.bump,
    )]
    pub bless_state: Box<Account<'info, BlessTokenState>>,

    #[account(mut)]
    pub bless_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_BLESS_TOKEN_META_STATE.as_bytes(), bless_state.key().as_ref()],
        bump = bless_meta_state.bump,
    )]
    pub bless_meta_state: Account<'info, BlessTokenMetaState>,

    /// CHECK: this is meta pda
    pub meta_pda: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    /// CHECK: metadata program
    pub metadata_program: AccountInfo<'info>,
}

impl<'info> BlessTokenMetaSetMetadata<'info> {
    pub fn set_metadata(&mut self) -> Result<()> {
        self.create_metadata()
    }

    pub fn create_metadata(&mut self) -> Result<()> {
        let rent = self.rent.to_account_info();
        let meta_ix = CreateMetadataAccountV3CpiAccounts {
            metadata: &self.meta_pda,
            mint: &self.bless_mint.to_account_info(),
            mint_authority: &self.bless_state.to_account_info(),
            payer: &self.payer.to_account_info(),
            update_authority: (&self.payer.to_account_info(), true),
            system_program: &self.system_program.to_account_info(),
            rent: Some(&rent),
        };
        let args = CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: "test".to_string(),
                symbol: "test".to_string(),
                uri: "test".to_string(),
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: self.bless_state.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        };

        let mint_bytes = self.bless_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED_BLESS_CONTRACT_STATE.as_bytes(),
            mint_bytes.as_ref(),
            &[self.bless_state.bump],
        ]];

        CreateMetadataAccountV3Cpi::new(&self.metadata_program.to_account_info(), meta_ix, args)
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
