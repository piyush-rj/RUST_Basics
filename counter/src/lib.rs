use std::fmt::{Display, Formatter, Result};
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    entrypoint
};

#[derive(BorshSerialize, BorshDeserialize)]
enum CounterInstructionType {
    Increment(u32), // [increment/decrement, 8, 8, 8, 8]
    Decrement(u32)
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Counter {
    count: u32,
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {    
        write!(f, "{}", self.count)
    }
}

entrypoint!(counter_contract);

pub fn counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],   
    instruction_data: &[u8]
) -> ProgramResult {

    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = CounterInstructionType::try_from_slice(instruction_data)?;

    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?; // deserializing -> reads the raw bytes and interprets it into struct Counter
    msg!("{:?}", counter_data);

    match instruction_type {
        CounterInstructionType::Increment(value) => {
            msg!("increasing count");
            counter_data.count += value;
        },
        CounterInstructionType::Decrement(value) => {
            msg!("decreasing count");
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?; // serializing -> converts the struct back to bytes
    msg!("contrsct passed");
    Ok(())
}