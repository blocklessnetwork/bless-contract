use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, Mint, MintTo, SetAuthority, Token, TokenAccount,
};

use crate::{
    BlessTokenState, SEED_BLESS_CONTRACT_STATE, WALLET_COMMUNITY_REWARDS_FEE,
    WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE, WALLET_FOUNDATION_FEE,
    WALLET_INVESTOR_FEE, WALLET_TEAM_ADVISOR_FEE,
};

#[derive(Accounts)]
pub struct InitBlessToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // current_authority is the current controller of the mint
    #[account(mut)]
    pub current_authority: Signer<'info>,

    #[account(mut)]
    pub bless_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        space = 8 + BlessTokenState::INIT_SPACE,
        bump,
    )]
    pub bless_state: Box<Account<'info, BlessTokenState>>,

    #[account(
        mut,
        constraint = bless_mint.key() == wallet_investor.mint,
    )]
    pub wallet_investor: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == wallet_team_advisor.mint,
    )]
    pub wallet_team_advisor: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == wallet_foundation.mint,
    )]
    pub wallet_foundation: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == wallet_ecosystem_liquidityprovision_tgtmarketing.mint,
    )]
    pub wallet_ecosystem_liquidityprovision_tgtmarketing: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == wallet_community_rewards.mint,
    )]
    pub wallet_community_rewards: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> InitBlessToken<'info> {
    /// Transfer mint authority to this program's PDA (program-derived address).
    /// Initially, the contract (this program) holds the mint authority after initialization.
    /// This step delegates full minting control to the program logic.
    fn transfer_control(&mut self) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            SetAuthority {
                current_authority: self.current_authority.to_account_info(),
                account_or_mint: self.bless_mint.to_account_info(),
            },
        );

        token::set_authority(
            cpi_ctx,
            AuthorityType::MintTokens,
            Some(self.bless_state.key()),
        )?;
        Ok(())
    }

    /// mint the token to the account.
    /// signer_seeds is the signature.
    fn mint_to(
        &mut self,
        to: AccountInfo<'info>,
        amount: u64,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.bless_mint.to_account_info(),
                to: to,
                authority: self.bless_state.to_account_info(),
            },
            signer_seeds,
        );
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    /// mint all token to wallet 1-5
    fn mint_to_all_to_wallets(&mut self) -> Result<()> {
        let bless_mint = self.bless_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED_BLESS_CONTRACT_STATE.as_bytes(),
            bless_mint.as_ref(),
            &[self.bless_state.bump],
        ]];
        self.mint_to(
            self.wallet_community_rewards.to_account_info(),
            WALLET_COMMUNITY_REWARDS_FEE,
            signer_seeds,
        )?;
        self.mint_to(
            self.wallet_foundation.to_account_info(),
            WALLET_FOUNDATION_FEE,
            signer_seeds,
        )?;
        self.mint_to(
            self.wallet_ecosystem_liquidityprovision_tgtmarketing
                .to_account_info(),
            WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE,
            signer_seeds,
        )?;
        self.mint_to(
            self.wallet_investor.to_account_info(),
            WALLET_INVESTOR_FEE,
            signer_seeds,
        )?;
        self.mint_to(
            self.wallet_team_advisor.to_account_info(),
            WALLET_TEAM_ADVISOR_FEE,
            signer_seeds,
        )?;
        Ok(())
    }

    /// signature guarntee the amount and recharge_time is not fake.
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.bless_state.set_inner(BlessTokenState {
            mint_pubkey: self.bless_mint.key(),
            bump,
            wallet_investor: self.wallet_investor.key(),
            wallet_team_advisor: self.wallet_team_advisor.key(),
            wallet_foundation: self.wallet_foundation.key(),
            wallet_ecosystem_liquidityprovision_tgtmarketing: self
                .wallet_ecosystem_liquidityprovision_tgtmarketing
                .key(),
            wallet_community_rewards: self.wallet_community_rewards.key(),
            wallet_investor_token: WALLET_INVESTOR_FEE,
            wallet_team_advisor_token: WALLET_TEAM_ADVISOR_FEE,
            wallet_foundation_token: WALLET_FOUNDATION_FEE,
            wallet_ecosystem_liquidityprovision_tgtmarketing_token:
                WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE,
            wallet_community_rewards_token: WALLET_COMMUNITY_REWARDS_FEE,
        });
        self.transfer_control()?;
        self.mint_to_all_to_wallets()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tables() {
        let base = 27;
        let mut rs = vec![];
        for i in (12..=33).step_by(3) {
            rs.push(base + i);
        }
        assert!(rs[0] == 39);
        assert!(rs[1] == 42);
        assert!(rs[2] == 45);
        assert!(rs[3] == 48);
        assert!(rs[4] == 51);
        assert!(rs[5] == 54);
        assert!(rs[6] == 57);
        assert!(rs[7] == 60);
    }
}
