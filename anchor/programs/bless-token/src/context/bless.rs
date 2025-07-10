use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, Mint, MintTo, SetAuthority, Token, TokenAccount,
};

use crate::{
    BitVec, BlessTokenState, RuleOutcome, BITMAP_SIZE, SEED_BLESS_CONTRACT_STATE, SEED_BLESS_VAULT,
    WALLET_COMMUNITY_REWARDS_FEE, WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE,
    WALLET_FOUNDATION_FEE, WALLET_INVESTOR_FEE, WALLET_TEAM_ADVISOR_FEE,
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
        seeds = [SEED_BLESS_VAULT.as_bytes(), bless_mint.key().as_ref()],
        token::mint = bless_mint,
        token::authority = bless_state,
        bump,
    )]
    pub vault_account: Box<Account<'info, TokenAccount>>,

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
        constraint = bless_mint.key() == preseed_sale.mint,
    )]
    pub preseed_sale: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == seed_sale.mint,
    )]
    pub seed_sale: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == advisors.mint,
    )]
    pub advisors: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == team.mint,
    )]
    pub team: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == ecosystem.mint,
    )]
    pub ecosystem: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == foundation.mint,
    )]
    pub foundation: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == liquidity_provision.mint,
    )]
    pub liquidity_provision: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == tgt_marketing.mint,
    )]
    pub tgt_marketing: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == airdrop.mint,
    )]
    pub airdrop: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == community_rewards.mint,
    )]
    pub community_rewards: Box<Account<'info, TokenAccount>>,

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

    /// set the funding rule.
    fn initial_rules(&mut self) -> Result<()> {
        // Following months need funding.
        for i in (12..=33).step_by(3) {
            // 12 = C39,15 = C42,18 = C45, 21 = C48,24 = C51, 27 = C54,30 = C57, 33 = C60
            self.bless_state
                .preseed_sale_rule_outcome
                .rule
                .set(i, true)?;
            // 12 = D39,15 = D42,18 = D45, 21 = D48,24 = D51, 27 = D54,30 = D57, 33 = D60
            self.bless_state.seed_sale_rule_outcome.rule.set(i, true)?;
        }

        for i in (12..=69).step_by(3) {
            // e39 - e96
            self.bless_state.advisors_rule_outcome.rule.set(i, true)?;
            self.bless_state.team_rule_outcome.rule.set(i, true)?;
        }

        // initial the ecosystem funding rule.
        self.bless_state.ecosystem_rule_outcome.rule.set(0, true)?;
        for i in (6..27).step_by(3) {
            self.bless_state.ecosystem_rule_outcome.rule.set(i, true)?;
        }

        // initial the foundation funding rule.
        for i in (6..60).step_by(6) {
            self.bless_state.foundation_rule_outcome.rule.set(i, true)?;
        }

        //initial the liquidity provision funding rule.
        self.bless_state
            .liquidity_provision_rule_outcome
            .rule
            .set(0, true)?;

        //initial the tgt marketing funding rule.
        self.bless_state
            .tgt_marketing_rule_outcome
            .rule
            .set(0, true)?;

        //initial the airdrop funding rule.
        self.bless_state.airdrop_rule_outcome.rule.set(0, true)?;

        //initial the community rewards funding rule.
        self.bless_state
            .community_rewards_rule_outcome
            .rule
            .set(0, true)?;

        Ok(())
    }

    /// mint the token to the account.
    /// signer_seeds is the signature.
    fn mint_to(&mut self, to: AccountInfo<'info>, amount: u64) -> Result<()> {
        let bless_mint = self.bless_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED_BLESS_CONTRACT_STATE.as_bytes(),
            bless_mint.as_ref(),
            &[self.bless_state.bump],
        ]];
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
        self.mint_to(
            self.wallet_community_rewards.to_account_info(),
            WALLET_COMMUNITY_REWARDS_FEE,
        )?;
        self.mint_to(
            self.wallet_foundation.to_account_info(),
            WALLET_FOUNDATION_FEE,
        )?;
        self.mint_to(
            self.wallet_ecosystem_liquidityprovision_tgtmarketing
                .to_account_info(),
            WALLET_ECOSYSTEM_LIQUIDITYPROVISION_TGTMARKETING_FEE,
        )?;
        self.mint_to(self.wallet_investor.to_account_info(), WALLET_INVESTOR_FEE)?;
        self.mint_to(
            self.wallet_team_advisor.to_account_info(),
            WALLET_TEAM_ADVISOR_FEE,
        )?;
        Ok(())
    }

    /// signature guarntee the amount and recharge_time is not fake.
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.bless_state.set_inner(BlessTokenState {
            mint_pubkey: self.bless_mint.key(),
            preseed_sale: self.preseed_sale.key(),
            preseed_sale_rule_outcome: Default::default(),
            seed_sale: self.seed_sale.key(),
            seed_sale_rule_outcome: Default::default(),
            advisors: self.advisors.key(),
            advisors_rule_outcome: Default::default(),
            team: self.team.key(),
            team_rule_outcome: Default::default(),
            ecosystem: self.ecosystem.key(),
            ecosystem_rule_outcome: Default::default(),
            foundation: self.foundation.key(),
            foundation_rule_outcome: Default::default(),
            liquidity_provision: self.liquidity_provision.key(),
            liquidity_provision_rule_outcome: Default::default(),
            tgt_marketing: self.tgt_marketing.key(),
            tgt_marketing_rule_outcome: Default::default(),
            airdrop: self.airdrop.key(),
            airdrop_rule_outcome: Default::default(),
            community_rewards: self.community_rewards.key(),
            community_rewards_rule_outcome: RuleOutcome {
                // Funding should be done every month.
                rule: BitVec::new_with_default(BITMAP_SIZE, 0xFF),
                outcome: BitVec::new(BITMAP_SIZE),
            },
            bump,
            current_month: 0,
            wallet_investor: self.wallet_investor.key(),
            wallet_team_advisor: self.wallet_team_advisor.key(),
            wallet_foundation: self.wallet_foundation.key(),
            wallet_ecosystem_liquidityprovision_tgtmarketing: self
                .wallet_ecosystem_liquidityprovision_tgtmarketing
                .key(),
            wallet_community_rewards: self.wallet_community_rewards.key(),
        });
        self.transfer_control()?;
        self.initial_rules()?;
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
