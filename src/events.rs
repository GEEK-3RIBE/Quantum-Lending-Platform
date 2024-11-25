use anchor_lang::prelude::*;

#[event]
pub struct Deposited {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Withdrawn {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Borrowed {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Repaid {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Liquidated {
    pub user: Pubkey,
    pub amount: u64,
}
