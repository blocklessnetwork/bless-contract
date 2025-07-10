use anchor_lang::error_code;

#[error_code]
pub enum BlsError {
    #[msg("Out of bound")]
    OutofBound,

    #[msg("Out of max month")]
    OufoutMaxMonth,
}
