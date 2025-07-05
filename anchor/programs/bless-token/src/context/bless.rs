use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct InitBlessToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub bless_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = bless_mint.key() == preseed_sale.mint,
    )]
    pub preseed_sale: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == seed_sale.mint,
    )]
    pub seed_sale: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == advisors.mint,
    )]
    pub advisors: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == team.mint,
    )]
    pub team: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == ecosystem.mint,
    )]
    pub ecosystem: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == foundation.mint,
    )]
    pub foundation: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == liquidity_provision.mint,
    )]
    pub liquidity_provision: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == tgt_marketing.mint,
    )]
    pub tgt_marketing: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == airdrop.mint,
    )]
    pub airdrop: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = bless_mint.key() == community_rewards.mint,
    )]
    pub community_rewards: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitBlessToken<'info> {
    /// signature guarntee the amount and recharge_time is not fake.
    pub fn init(&mut self) -> Result<()> {
        Ok(())
    }
}
