// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Transfer};
// use crate::{contexts::*, errors::*, events::*, state::*};

// pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let lending_pool = &mut ctx.accounts.lending_pool;

//     let cpi_accounts = Transfer {
//         from: ctx.accounts.user_token_account.to_account_info(),
//         to: ctx.accounts.pool_token_account.to_account_info(),
//         authority: ctx.accounts.user.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

//     user_account.collateral += amount;
//     lending_pool.total_deposits += amount;

//     emit!(Deposited {
//         user: ctx.accounts.user.key(),
//         amount,
//     });

//     Ok(())
// }

// pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let lending_pool = &mut ctx.accounts.lending_pool;

//     require!(
//         user_account.collateral >= amount,
//         LendingPoolError::InsufficientCollateral
//     );

//     let cpi_accounts = Transfer {
//         from: ctx.accounts.pool_token_account.to_account_info(),
//         to: ctx.accounts.user_token_account.to_account_info(),
//         authority: ctx.accounts.pool_signer.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

//     user_account.collateral -= amount;
//     lending_pool.total_deposits -= amount;

//     emit!(Withdrawn {
//         user: ctx.accounts.user.key(),
//         amount,
//     });

//     Ok(())
// }

// pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let lending_pool = &mut ctx.accounts.lending_pool;

//     let collateral_value = user_account.collateral;
//     let borrow_limit = collateral_value * lending_pool.collateral_factor as u64 / 100;
//     require!(
//         user_account.debt + amount <= borrow_limit,
//         LendingPoolError::BorrowLimitExceeded
//     );

//     let cpi_accounts = Transfer {
//         from: ctx.accounts.pool_token_account.to_account_info(),
//         to: ctx.accounts.user_token_account.to_account_info(),
//         authority: ctx.accounts.pool_signer.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

//     user_account.debt += amount;
//     lending_pool.total_borrows += amount;

//     emit!(Borrowed {
//         user: ctx.accounts.user.key(),
//         amount,
//     });

//     Ok(())
// }

// pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let lending_pool = &mut ctx.accounts.lending_pool;

//     require!(
//         user_account.debt >= amount,
//         LendingPoolError::IncorrectRepaymentAmount
//     );

//     let cpi_accounts = Transfer {
//         from: ctx.accounts.user_token_account.to_account_info(),
//         to: ctx.accounts.pool_token_account.to_account_info(),
//         authority: ctx.accounts.user.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

//     user_account.debt -= amount;
//     lending_pool.total_borrows -= amount;

//     emit!(Repaid {
//         user: ctx.accounts.user.key(),
//         amount,
//     });

//     Ok(())
// }

// pub fn liquidate(ctx: Context<Liquidate>, target_user: Pubkey) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let lending_pool = &mut ctx.accounts.lending_pool;

//     require!(
//         user_account.debt > user_account.collateral,
//         LendingPoolError::LiquidationNotEligible
//     );

//     let cpi_accounts = Transfer {
//         from: ctx.accounts.user_token_account.to_account_info(),
//         to: ctx.accounts.pool_token_account.to_account_info(),
//         authority: ctx.accounts.user.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     token::transfer(
//         CpiContext::new(cpi_program, cpi_accounts),
//         user_account.collateral,
//     )?;

//     user_account.collateral = 0;
//     user_account.debt = 0;

//     emit!(Liquidated {
//         user: target_user,
//         amount: user_account.collateral,
//     });

//     Ok(())
// }
