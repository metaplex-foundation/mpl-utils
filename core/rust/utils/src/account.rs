use arch_program::{
    account::AccountInfo,
    bitcoin::{self, consensus, ScriptBuf, Transaction, TxOut},
    entrypoint::ProgramResult,
    helper::{add_state_transition, get_state_transition_tx},
    msg,
    program::{get_account_script_pubkey, get_bitcoin_tx},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

/// Create account for Arch Network, adapted from Solana's approach
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    // Get the current transaction
    let mut tx = get_state_transition_tx(&[
        payer_info.clone(),
        new_account_info.clone(),
        system_program_info.clone(),
    ]);

    msg!("Creating a new account of size {}", size);

    // Create the account using system instruction
    invoke_signed(
        &system_instruction::create_account(
            new_account_info.utxo.txid().try_into().unwrap(),
            new_account_info.utxo.vout(),
            *new_account_info.key,
        ),
        &[new_account_info.clone()],
        &[signer_seeds],
    )?;

    // Initialize data with zeros using realloc
    new_account_info.realloc(size, false)?;

    // Update the transaction with state transition for the new account
    add_state_transition(&mut tx, new_account_info);

    // Add appropriate UTXO output
    let script_pubkey_bytes = get_account_script_pubkey(new_account_info.key);

    // Get the UTXO information
    let utxo_tx: Transaction = consensus::deserialize(
        &get_bitcoin_tx(new_account_info.utxo.txid().try_into().unwrap()).unwrap(),
    )
    .unwrap();

    // Add the UTXO value to our output
    tx.output.push(TxOut {
        value: utxo_tx.output[new_account_info.utxo.vout() as usize].value,
        script_pubkey: ScriptBuf::from_bytes(script_pubkey_bytes.to_vec()),
    });

    Ok(())
}

/// Resize an account in Arch Network
pub fn resize_or_reallocate_account_raw<'a>(
    target_account: &AccountInfo<'a>,
    funding_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    new_size: usize,
) -> ProgramResult {
    // Get current size without using borrow which requires error conversion
    let current_size = target_account
        .data
        .try_borrow()
        .map_err(|_| ProgramError::Custom(1))?
        .len();

    if new_size == current_size {
        return Ok(());
    }

    msg!("Resizing account to {}", new_size);

    // In Arch Network, we need to create a transaction to resize
    let mut tx = get_state_transition_tx(&[
        funding_account.clone(),
        target_account.clone(),
        system_program.clone(),
    ]);

    // Resize the target account's data
    target_account.realloc(new_size, false)?;

    // Update the transaction with state transition
    add_state_transition(&mut tx, target_account);

    // Handle the UTXO output for the resized account
    let script_pubkey_bytes = get_account_script_pubkey(target_account.key);

    // Get the UTXO information
    let utxo_tx: Transaction = consensus::deserialize(
        &get_bitcoin_tx(target_account.utxo.txid().try_into().unwrap()).unwrap(),
    )
    .unwrap();

    // Add the UTXO value to our output
    tx.output.push(TxOut {
        value: utxo_tx.output[target_account.utxo.vout() as usize].value,
        script_pubkey: ScriptBuf::from_bytes(script_pubkey_bytes.to_vec()),
    });

    Ok(())
}

/// Close an account in Arch Network
pub fn close_account_raw<'a>(
    dest_account_info: &AccountInfo<'a>,
    src_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    msg!("Closing account");

    // Create a transaction that transfers the value from src to dest
    let mut tx = get_state_transition_tx(&[dest_account_info.clone(), src_account_info.clone()]);

    // Reset the source account by reallocating to zero size
    src_account_info.realloc(0, false)?;

    // Update the transaction with state transition
    add_state_transition(&mut tx, src_account_info);

    // Transfer the UTXO value to the destination account
    let dest_script_pubkey = get_account_script_pubkey(dest_account_info.key);

    // Get the UTXO information from source account
    let utxo_tx: Transaction = consensus::deserialize(
        &get_bitcoin_tx(src_account_info.utxo.txid().try_into().unwrap()).unwrap(),
    )
    .unwrap();

    // Add an output to transfer the value to the destination account
    tx.output.push(TxOut {
        value: utxo_tx.output[src_account_info.utxo.vout() as usize].value,
        script_pubkey: ScriptBuf::from_bytes(dest_script_pubkey.to_vec()),
    });

    Ok(())
}
