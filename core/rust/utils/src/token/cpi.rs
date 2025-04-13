use arch_program::{
    account::{AccountInfo, AccountMeta},
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::token::assertions::APL_TOKEN_PROGRAM_ID_BYTES;

// APL token instruction enum
enum TokenInstruction {
    Burn = 3,
    CloseAccount = 9,
    MintTo = 7,
    Transfer = 2,
    TransferChecked = 12,
}

// Helper function to create APL token program instructions
fn create_token_instruction(
    instruction_type: TokenInstruction,
    token_program: &Pubkey,
    accounts: Vec<AccountMeta>,
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *token_program,
        accounts,
        data,
    }
}

// Create a burn instruction for the APL token program
fn burn_instruction(
    token_program: &Pubkey,
    source: &Pubkey,
    mint: &Pubkey,
    authority: &Pubkey,
    signers: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*source, false),
        AccountMeta::new(*mint, false),
        AccountMeta::new(*authority, signers.is_empty()),
    ];

    // Add additional signers if provided
    for signer in signers.iter() {
        accounts.push(AccountMeta::new(**signer, true));
    }

    // Serialize instruction data: [instruction_byte, amount_bytes]
    let mut data = vec![TokenInstruction::Burn as u8];
    data.extend_from_slice(&amount.to_le_bytes());

    Ok(create_token_instruction(
        TokenInstruction::Burn,
        token_program,
        accounts,
        data,
    ))
}

// Create a close account instruction for the APL token program
fn close_account_instruction(
    token_program: &Pubkey,
    account: &Pubkey,
    destination: &Pubkey,
    owner: &Pubkey,
    signers: &[&Pubkey],
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*account, false),
        AccountMeta::new(*destination, false),
        AccountMeta::new(*owner, signers.is_empty()),
    ];

    // Add additional signers if provided
    for signer in signers.iter() {
        accounts.push(AccountMeta::new(**signer, true));
    }

    // Serialize instruction data: [instruction_byte]
    let data = vec![TokenInstruction::CloseAccount as u8];

    Ok(create_token_instruction(
        TokenInstruction::CloseAccount,
        token_program,
        accounts,
        data,
    ))
}

// Create a mint-to instruction for the APL token program
fn mint_to_instruction(
    token_program: &Pubkey,
    mint: &Pubkey,
    destination: &Pubkey,
    authority: &Pubkey,
    signers: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*mint, false),
        AccountMeta::new(*destination, false),
        AccountMeta::new(*authority, signers.is_empty()),
    ];

    // Add additional signers if provided
    for signer in signers.iter() {
        accounts.push(AccountMeta::new(**signer, true));
    }

    // Serialize instruction data: [instruction_byte, amount_bytes]
    let mut data = vec![TokenInstruction::MintTo as u8];
    data.extend_from_slice(&amount.to_le_bytes());

    Ok(create_token_instruction(
        TokenInstruction::MintTo,
        token_program,
        accounts,
        data,
    ))
}

// Create a transfer instruction for the APL token program
fn transfer_instruction(
    token_program: &Pubkey,
    source: &Pubkey,
    destination: &Pubkey,
    authority: &Pubkey,
    signers: &[&Pubkey],
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*source, false),
        AccountMeta::new(*destination, false),
        AccountMeta::new(*authority, signers.is_empty()),
    ];

    // Add additional signers if provided
    for signer in signers.iter() {
        accounts.push(AccountMeta::new(**signer, true));
    }

    // Serialize instruction data: [instruction_byte, amount_bytes]
    let mut data = vec![TokenInstruction::Transfer as u8];
    data.extend_from_slice(&amount.to_le_bytes());

    Ok(create_token_instruction(
        TokenInstruction::Transfer,
        token_program,
        accounts,
        data,
    ))
}

// Create a transfer-checked instruction for the APL token program
fn transfer_checked_instruction(
    token_program: &Pubkey,
    source: &Pubkey,
    mint: &Pubkey,
    destination: &Pubkey,
    authority: &Pubkey,
    signers: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*source, false),
        AccountMeta::new(*mint, false),
        AccountMeta::new(*destination, false),
        AccountMeta::new(*authority, signers.is_empty()),
    ];

    // Add additional signers if provided
    for signer in signers.iter() {
        accounts.push(AccountMeta::new(**signer, true));
    }

    // Serialize instruction data: [instruction_byte, amount_bytes, decimals_byte]
    let mut data = vec![TokenInstruction::TransferChecked as u8];
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(decimals);

    Ok(create_token_instruction(
        TokenInstruction::TransferChecked,
        token_program,
        accounts,
        data,
    ))
}

pub fn spl_token_burn(params: TokenBurnParams<'_, '_>) -> ProgramResult {
    let TokenBurnParams {
        mint,
        source,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;
    let mut seeds: Vec<&[&[u8]]> = vec![];
    if let Some(seed) = authority_signer_seeds {
        seeds.push(seed);
    }
    invoke_signed(
        &burn_instruction(
            token_program.key,
            source.key,
            mint.key,
            authority.key,
            &[authority.key],
            amount,
        )?,
        &[source, mint, authority],
        seeds.as_slice(),
    )
}

/// TokenBurnParams
pub struct TokenBurnParams<'a: 'b, 'b> {
    /// mint
    pub mint: AccountInfo<'a>,
    /// source
    pub source: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

pub fn spl_token_close(params: TokenCloseParams<'_, '_>) -> ProgramResult {
    let TokenCloseParams {
        account,
        destination,
        owner,
        authority_signer_seeds,
        token_program,
    } = params;
    let mut seeds: Vec<&[&[u8]]> = vec![];
    if let Some(seed) = authority_signer_seeds {
        seeds.push(seed);
    }
    invoke_signed(
        &close_account_instruction(
            token_program.key,
            account.key,
            destination.key,
            owner.key,
            &[],
        )?,
        &[account, destination, owner, token_program],
        seeds.as_slice(),
    )
}

/// TokenCloseParams
pub struct TokenCloseParams<'a: 'b, 'b> {
    /// Token account
    pub account: AccountInfo<'a>,
    /// Destination for redeemed SOL.
    pub destination: AccountInfo<'a>,
    /// Owner of the token account.
    pub owner: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

pub fn spl_token_mint_to(params: TokenMintToParams<'_, '_>) -> ProgramResult {
    let TokenMintToParams {
        mint,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;
    let mut seeds: Vec<&[&[u8]]> = vec![];
    if let Some(seed) = authority_signer_seeds {
        seeds.push(seed);
    }
    invoke_signed(
        &mint_to_instruction(
            token_program.key,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[mint, destination, authority, token_program],
        seeds.as_slice(),
    )
}

/// TokenMintToParams
pub struct TokenMintToParams<'a: 'b, 'b> {
    /// mint
    pub mint: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[allow(deprecated)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        amount,
        authority,
        token_program,
        authority_signer_seeds,
        ..
    } = params;
    let seeds = if let Some(seeds) = authority_signer_seeds {
        seeds
    } else {
        &[]
    };

    invoke_signed(
        &transfer_instruction(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[authority.key],
            amount,
        )?,
        &[source, destination, authority],
        &[seeds],
    )
}

/// TokenTransferParams
#[derive(Debug)]
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// mint
    pub mint: AccountInfo<'a>,
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

pub fn spl_token_transfer_checked(params: TokenTransferCheckedParams<'_, '_>) -> ProgramResult {
    let TokenTransferCheckedParams {
        mint,
        source,
        destination,
        amount,
        authority,
        token_program,
        authority_signer_seeds,
        decimals,
    } = params;
    let seeds = if let Some(seeds) = authority_signer_seeds {
        seeds
    } else {
        &[]
    };

    invoke_signed(
        &transfer_checked_instruction(
            token_program.key,
            source.key,
            mint.key,
            destination.key,
            authority.key,
            &[authority.key],
            amount,
            decimals,
        )?,
        &[source, mint, destination, authority],
        &[seeds],
    )
}

/// TokenTransferParams
#[derive(Debug)]
pub struct TokenTransferCheckedParams<'a: 'b, 'b> {
    /// mint
    pub mint: AccountInfo<'a>,
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: Option<&'b [&'b [u8]]>,
    /// token_program
    pub token_program: AccountInfo<'a>,
    /// decimals
    pub decimals: u8,
}
