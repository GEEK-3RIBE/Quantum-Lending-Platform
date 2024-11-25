use anchor_lang::prelude::*;

#[account]
pub struct LendingPool {
    pub liquidity: u64,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub collateral_factor: u8, // e.g., 75 means 75%
    pub bump: u8,
}

#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub collateral: u64,
    pub debt: u64,
}
