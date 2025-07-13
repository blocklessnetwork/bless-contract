use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{
    errors::BlsError, BlessTokenState, COMMUNITY_AIRDROP_FUND_FEE, COMMUNITY_INCENTIVES_FUN_FEE,
    MARKET_MAKING_FUND_FEE, MAX_MONTHS, PRE_SEED_SALE_FUND_FEE, SEED_BLESS_CONTRACT_STATE,
    SEED_BLESS_VAULT, TGE_MARKETING_FUND_FEE,
};

#[derive(Accounts)]
pub struct FundBlessToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_BLESS_CONTRACT_STATE.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.bump,
    )]
    pub bless_state: Box<Account<'info, BlessTokenState>>,

    #[account(
        mut,
        seeds = [SEED_BLESS_VAULT.as_bytes(), bless_mint.key().as_ref()],
        bump = bless_state.vault_bump,
    )]
    pub vault_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == bless_state.mint_pubkey,
    )]
    pub bless_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = bless_mint.key() == preseed_sale.mint,
        constraint = preseed_sale.key() == bless_state.preseed_sale,
    )]
    pub preseed_sale: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == seed_sale.mint,
        constraint = seed_sale.key() == bless_state.seed_sale,
    )]
    pub seed_sale: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == advisors.mint,
        constraint = advisors.key() == bless_state.advisors,
    )]
    pub advisors: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == team.mint,
        constraint = team.key() == bless_state.team,
    )]
    pub team: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == ecosystem.mint,
        constraint = ecosystem.key() == bless_state.ecosystem,
    )]
    pub ecosystem: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == foundation.mint,
        constraint = foundation.key() == bless_state.foundation,
    )]
    pub foundation: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == market_making.mint,
        constraint = market_making.key() == bless_state.market_making,
    )]
    pub market_making: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == tgt_marketing.mint,
        constraint = tgt_marketing.key() == bless_state.tgt_marketing,
    )]
    pub tgt_marketing: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == community_airdrop.mint,
        constraint = community_airdrop.key() == bless_state.community_airdrop,
    )]
    pub community_airdrop: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bless_mint.key() == community_incentives.mint,
        constraint = community_incentives.key() == bless_state.community_incentives,
    )]
    pub community_incentives: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> FundBlessToken<'info> {
    pub fn transfer(&self, to: AccountInfo<'info>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.bless_mint.to_account_info(),
            Transfer {
                from: self.vault_account.to_account_info(),
                to: to,
                authority: self.bless_state.to_account_info(),
            },
        );

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    /// fund genesis token to packet accounts, for the rule only fund once.
    pub fn fund_genesis(&mut self) -> Result<()> {
        self.transfer(self.market_making.to_account_info(), MARKET_MAKING_FUND_FEE)?;
        self.bless_state.market_making_flag = true;

        self.transfer(self.tgt_marketing.to_account_info(), TGE_MARKETING_FUND_FEE)?;
        self.bless_state.tgt_marketing_flag = true;

        self.transfer(
            self.community_airdrop.to_account_info(),
            COMMUNITY_AIRDROP_FUND_FEE,
        )?;
        self.bless_state.community_airdrop_flag = true;

        self.transfer(
            self.community_incentives.to_account_info(),
            COMMUNITY_INCENTIVES_FUN_FEE,
        )?;
        self.bless_state.community_incentives_flag = true;
        self.bless_state.current_month = 1;
        Ok(())
    }

    /// fund period, which setup the rule when initial.
    pub fn fund_by_rule(&mut self) -> Result<()> {
        let idx = self.bless_state.current_month as usize;
        macro_rules! fund {
            ($ro: ident, $a: ident, $fee: ident) => {
                if self.bless_state.$ro.outcome.get(idx)? {
                    self.transfer(self.$a.to_account_info(), PRE_SEED_SALE_FUND_FEE)?;
                    self.bless_state.$ro.outcome.set(idx, true)?;
                }
            };
        }
        fund!(
            preseed_sale_rule_outcome,
            preseed_sale,
            PRE_SEED_SALE_FUND_FEE
        );

        fund!(seed_sale_rule_outcome, seed_sale, SEED_SALE_FUND_FEE);
        fund!(advisors_rule_outcome, advisors, ADVISORS_FUND_FEE);
        fund!(team_rule_outcome, team, TEAM_FUND_FEE);
        fund!(ecosystem_rule_outcome, ecosystem, ECOSYSTEM_FUND_FEE);
        fund!(foundation_rule_outcome, foundation, FOUNDATION_FEE);
        self.bless_state.current_month += 1;
        Ok(())
    }

    pub fn fund(&mut self) -> Result<()> {
        if self.bless_state.current_month >= MAX_MONTHS as _ {
            return Err(BlsError::OutOfMaxMonth.into());
        }
        if self.bless_state.current_month == 0 {
            self.fund_genesis()?;
        } else {
            self.fund_by_rule()?;
        }
        Ok(())
    }
}
