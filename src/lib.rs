use cvlr::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction,
};

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
