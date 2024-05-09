use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    
    #[msg("Attempting to withdraw from vault with account that is not the authority")]
    WrongVaultAuthority,

    #[msg("The project account provided is invalid")]
    ProjectAccountInvalid,

    #[msg("This mint is not current supported")]
    MintNotSupported,

}