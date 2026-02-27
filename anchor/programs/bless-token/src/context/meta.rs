use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        update_metadata_accounts_v2, CreateMetadataAccountsV3, Metadata, UpdateMetadataAccountsV2,
    },
    token::{self, spl_token::instruction::AuthorityType, Mint, SetAuthority, Token},
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
    #[account(mut)]
    pub meta_pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub metadata_program: Program<'info, Metadata>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> BlessTokenMetaSetMetadata<'info> {
    pub fn update_metadata(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        let mint_bytes = self.bless_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED_BLESS_CONTRACT_STATE.as_bytes(),
            mint_bytes.as_ref(),
            &[self.bless_state.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.metadata_program.to_account_info().clone(),
            UpdateMetadataAccountsV2 {
                metadata: self.meta_pda.to_account_info(),
                update_authority: self.bless_state.to_account_info(),
            },
            signer_seeds,
        );
        let datav2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        update_metadata_accounts_v2(cpi_ctx, None, Some(datav2), None, Some(true))
    }
    pub fn create_metadata(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        let mint_bytes = self.bless_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED_BLESS_CONTRACT_STATE.as_bytes(),
            mint_bytes.as_ref(),
            &[self.bless_state.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.metadata_program.to_account_info().clone(),
            CreateMetadataAccountsV3 {
                metadata: self.meta_pda.to_account_info(),
                mint: self.bless_mint.to_account_info(),
                mint_authority: self.bless_state.to_account_info(),
                payer: self.payer.to_account_info(),
                update_authority: self.bless_state.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
            signer_seeds,
        );
        let datav2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(cpi_ctx, datav2, true, false, None)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BlessTokenDisableMint<'info> {
    #[account(
        mut,
        // only admin can disable mint authority.
        constraint = payer.key() == bless_meta_state.admin
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub bless_mint: Account<'info, Mint>,

    /// current authority of the mint
    #[account(mut)]
    pub current_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.bump,
    )]
    pub bless_state: Account<'info, BlessTokenState>,

    #[account(
        mut,
        seeds = [SEED_BLESS_TOKEN_META_STATE.as_bytes(), bless_state.key().as_ref()],
        bump = bless_meta_state.bump,
    )]
    pub bless_meta_state: Account<'info, BlessTokenMetaState>,

    pub token_program: Program<'info, Token>,
}

impl<'info> BlessTokenDisableMint<'info> {
    pub fn disable_mint(&mut self) -> Result<()> {
        let bless_token_key = Pubkey::from_str_const(crate::MINT_KEY);
        if self.bless_mint.key() != bless_token_key {
            return Err(BlsError::InvalidMintToken.into());
        }
        let mint_authority = self
            .bless_mint
            .mint_authority
            .ok_or(BlsError::MintAuthorityAlreadyDisabled)?;
        if mint_authority != self.current_authority.key() {
            return Err(BlsError::InvalidMintAuthority.into());
        }
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            SetAuthority {
                current_authority: self.current_authority.to_account_info(),
                account_or_mint: self.bless_mint.to_account_info(),
            },
        );
        token::set_authority(cpi_ctx, AuthorityType::MintTokens, None)?;
        Ok(())
    }
}
