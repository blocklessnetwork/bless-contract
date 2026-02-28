use anchor_lang::error_code;

#[error_code]
pub enum BlsError {
    #[msg("Out of bound")]
    OutOfBound,

    #[msg("Out of max month")]
    OutOfMaxMonth,

    #[msg("Invalid mint token.")]
    InvalidMintToken,

    #[msg("Invalid pending admin.")]
    InvalidPendingAdmin,

    #[msg("Invalid mint authority.")]
    InvalidMintAuthority,

    #[msg("Mint authority already disabled.")]
    MintAuthorityAlreadyDisabled,
}
