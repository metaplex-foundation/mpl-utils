use arch_program::{
    account::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use crate::assert_initialized;

// APL token program ID: apl-token00000000000000000000000
pub const APL_TOKEN_PROGRAM_ID_BYTES: [u8; 32] = [
    97, 112, 108, 45, 116, 111, 107, 101, 110, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
    48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
];

/// Arch Program Library Token Account structure
#[derive(Debug, Default, Clone)]
pub struct TokenAccount {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account
    pub owner: Pubkey,
    /// The amount of tokens this account holds
    pub amount: u64,
    /// If `true`, this account cannot be closed
    pub is_frozen: bool,
    /// If `true`, this account has been initialized
    pub is_initialized: bool,
}

// Implement the Sealed trait as required by Pack
impl Sealed for TokenAccount {}

impl Pack for TokenAccount {
    const LEN: usize = 32 + 32 + 8 + 1 + 1; // mint + owner + amount + is_frozen + is_initialized

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        // Extract mint Pubkey bytes (first 32 bytes)
        let mut mint_bytes = [0u8; 32];
        mint_bytes.copy_from_slice(&src[0..32]);
        let mint = Pubkey::from_slice(&mint_bytes);

        // Extract owner Pubkey bytes (next 32 bytes)
        let mut owner_bytes = [0u8; 32];
        owner_bytes.copy_from_slice(&src[32..64]);
        let owner = Pubkey::from_slice(&owner_bytes);

        // Extract amount (next 8 bytes)
        let mut amount_bytes = [0u8; 8];
        amount_bytes.copy_from_slice(&src[64..72]);
        let amount = u64::from_le_bytes(amount_bytes);

        // Extract flags
        let is_frozen = src[72] != 0;
        let is_initialized = src[73] != 0;

        Ok(TokenAccount {
            mint,
            owner,
            amount,
            is_frozen,
            is_initialized,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        if dst.len() < Self::LEN {
            panic!("Destination slice is too short");
        }

        // Copy mint Pubkey bytes
        dst[0..32].copy_from_slice(self.mint.as_ref());

        // Copy owner Pubkey bytes
        dst[32..64].copy_from_slice(self.owner.as_ref());

        // Copy amount bytes
        dst[64..72].copy_from_slice(&self.amount.to_le_bytes());

        // Set flags
        dst[72] = if self.is_frozen { 1 } else { 0 };
        dst[73] = if self.is_initialized { 1 } else { 0 };
    }
}

impl IsInitialized for TokenAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

pub trait ToTokenAccount {
    fn to_token_account(self) -> TokenAccount;
}

impl ToTokenAccount for AccountInfo<'_> {
    fn to_token_account(self) -> TokenAccount {
        assert_initialized(&self, ProgramError::UninitializedAccount).unwrap()
    }
}

impl ToTokenAccount for TokenAccount {
    fn to_token_account(self) -> TokenAccount {
        self
    }
}

pub fn assert_token_program_matches_package(
    token_program_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    // Create the APL token ID from hardcoded bytes
    let apl_token_id = Pubkey::from_slice(&APL_TOKEN_PROGRAM_ID_BYTES);

    if token_program_info.key == &apl_token_id {
        Ok(())
    } else {
        Err(error.into())
    }
}

/// Asserts that
/// * the given token account is initialized
/// * it's owner matches the provided owner
/// * it's mint matches the provided mint
/// * it holds more than than 0 tokens of the given mint.
/// Accepts either an &AccountInfo or an Account for token_account parameter.
pub fn assert_holder(
    token_account: impl ToTokenAccount,
    owner_info: &AccountInfo,
    mint_info: &AccountInfo,
    error: impl Into<ProgramError> + Clone,
) -> ProgramResult {
    let token_account: TokenAccount = token_account.to_token_account();

    if token_account.owner != *owner_info.key {
        return Err(error.into());
    }

    if token_account.mint != *mint_info.key {
        return Err(error.into());
    }

    if token_account.amount == 0 {
        return Err(error.into());
    }

    Ok(())
}
