use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Numerical Overflow")]
    NumericalOverflow,
    #[msg("Context is missing required bump")]
    MissingBump,
    #[msg("Missing either tip percentage or tippees array")]
    TooManyTippees,
    #[msg("Combined total shares for all tippees must add up to 10000")]
    InvalidTotalTippeeShare,
    #[msg("Invalid tip percentage, please provide value in basis points")]
    InvalidTipPercentage,
}
