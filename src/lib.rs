use cvlr::*;
use cvlr_solana::token::spl_token_account_get_amount;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[inline(never)]
#[cvlr::early_panic]
pub fn transfer_checked(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    spl_token::check_program_account(token_program_id)?;
    let data =
        spl_token::instruction::TokenInstruction::TransferChecked { amount, decimals }.pack();

    let accounts = vec![];
    // let mut accounts = Vec::with_capacity(4 + signer_pubkeys.len());
    // accounts.push(AccountMeta::new(*source_pubkey, false));
    // accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    // accounts.push(AccountMeta::new(*destination_pubkey, false));
    // accounts.push(AccountMeta::new_readonly(
    //     *authority_pubkey,
    //     signer_pubkeys.is_empty(),
    // ));
    // for signer_pubkey in signer_pubkeys.iter() {
    //     accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    // }

    // cvlr_assume!(*token_program_id == spl_token::id());
    let mut pubkey = Pubkey::new(&[0u8; 32]);
    unsafe {
        // Get a mutable pointer to the first byte
        let ptr = &mut pubkey as *mut Pubkey as *mut u64;
        // Write u64s directly
        *ptr.add(0) = 10637895772709248262u64;
        *ptr.add(1) = 12428223917890587609u64;
        *ptr.add(2) = 10463932726783620124u64;
        *ptr.add(3) = 12178014311288245306u64;
    }
    Ok(Instruction {
        program_id: pubkey,
        accounts,
        data,
    })
}

#[inline(never)]
#[cvlr::early_panic]
pub fn cvlr_invoke_transfer_checked(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
) -> ProgramResult {
    cvlr_assert!(account_infos.len() == 4);
    cvlr_assert!(instruction.data.len() > 0);
    cvlr_assert!(instruction.program_id == spl_token::id());
    let src_info = &account_infos[0];
    let dst_info = &account_infos[2];
    let authority_info = &account_infos[3];
    let amount = u64::from_le_bytes(
        instruction.data[1..9]
            .try_into()
            .expect("Invalid slice length"),
    );
    cvlr_solana::token::spl_token_transfer(src_info, dst_info, authority_info, amount)
}

#[macro_export]
macro_rules! cvlr_solana_init {
    ($wrapper_name:ident) => {
        fn $wrapper_name() {
            use cpivlr::*;
            rule_to_compile_transfer_token();
        }
    };
}

pub fn process_transfer_token_3(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let from = &accounts[1];
    let mint = &accounts[2];
    let to = &accounts[3];
    let authority = &accounts[4];
    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );
    let instruction = cvlr_transfer_checked(
        token_program.key,
        from.key,
        mint.key,
        to.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    let account_infos = vec![from.clone(), mint.clone(), to.clone(), authority.clone()];
    cvlr_invoke_transfer_checked(&instruction, &account_infos)?;
    Ok(())
}

#[rule]
pub fn rule_to_compile_transfer_token() {
    let account_infos = cvlr_solana::cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let from: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let to: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _authority: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());
    cvlr_assume!(from.key != to.key);
    let from_wallet_amount_pre = spl_token_account_get_amount(from);
    let to_wallet_amount_pre = spl_token_account_get_amount(to);
    process_transfer_token_3(&account_infos, &token_instruction_data).unwrap();
    let from_wallet_amount_post = spl_token_account_get_amount(from);
    let to_wallet_amount_post = spl_token_account_get_amount(to);
    cvlr_assert!(*token_program.key == spl_token::id());
    cvlr_assert!(from_wallet_amount_post == from_wallet_amount_pre - amount);
    cvlr_assert!(to_wallet_amount_post == to_wallet_amount_pre + amount);
}

#[inline(never)]
#[cvlr::early_panic]
pub fn cvlr_transfer_checked(
    token_program_id: &Pubkey,
    _source_pubkey: &Pubkey,
    _mint_pubkey: &Pubkey,
    _destination_pubkey: &Pubkey,
    _authority_pubkey: &Pubkey,
    _signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    spl_token::check_program_account(token_program_id)?;
    let data =
        spl_token::instruction::TokenInstruction::TransferChecked { amount, decimals }.pack();

    let accounts = vec![];
    // let mut accounts = Vec::with_capacity(4 + signer_pubkeys.len());
    // accounts.push(AccountMeta::new(*source_pubkey, false));
    // accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    // accounts.push(AccountMeta::new(*destination_pubkey, false));
    // accounts.push(AccountMeta::new_readonly(
    //     *authority_pubkey,
    //     signer_pubkeys.is_empty(),
    // ));
    // for signer_pubkey in signer_pubkeys.iter() {
    //     accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    // }

    // cvlr_assume!(*token_program_id == spl_token::id());
    let mut pubkey = Pubkey::new(&[0u8; 32]);
    unsafe {
        // Get a mutable pointer to the first byte
        let ptr = &mut pubkey as *mut Pubkey as *mut u64;
        // Write u64s directly
        *ptr.add(0) = 1u64;
        *ptr.add(1) = 2u64;
        *ptr.add(2) = 3u64;
        *ptr.add(3) = 4u64;
    }
    Ok(Instruction {
        program_id: pubkey,
        accounts,
        data,
    })
}
