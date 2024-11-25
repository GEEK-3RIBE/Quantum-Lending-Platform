use anchor_lang::prelude::*;

#[error_code]
pub enum LendingPoolError {
    #[msg("Insufficient collateral.")]
    InsufficientCollateral,

    #[msg("Borrow limit exceeded.")]
    BorrowLimitExceeded,

    #[msg("Incorrect repayment amount.")]
    IncorrectRepaymentAmount,

    #[msg("Liquidation not eligible.")]
    LiquidationNotEligible,
}
