use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint, 
    account_info::{
        next_account_info,
        AccountInfo
    }, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey
};

#[derive(BorshSerialize, BorshDeserialize)]
struct OnChainData {
    count: u32
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?;

    let mut counter_state = OnChainData::try_from_slice(&mut *data_account.data.borrow_mut())?;
    if counter_state.count == 0 {
        counter_state.count = 1;
    } else {
        counter_state.count = counter_state.count * 2;
    }

    counter_state.serialize(&mut *data_account.data.borrow_mut());

    Ok(())
}