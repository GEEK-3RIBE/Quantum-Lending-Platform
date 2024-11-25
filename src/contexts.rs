use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::state::*;

/// Context for the Deposit instruction
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Context for the Withdraw instruction
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"pool_signer"], bump = lending_pool.bump)]
    pub pool_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

/// Context for the Borrow instruction
#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"pool_signer"], bump = lending_pool.bump)]
    pub pool_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

/// Context for the Repay instruction
#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"pool_signer"], bump = lending_pool.bump)]
    pub pool_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

/// Context for the Liquidate instruction
#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"pool_signer"], bump = lending_pool.bump)]
    pub pool_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
