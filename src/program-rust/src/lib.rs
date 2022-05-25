// import crate to help serialize and deserialize data
use borsh::{BorshDeserialize, BorshSerialize};
// import important modules from solana program crate
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
// import the instructions module
pub mod instruction;
// use the contract instructions enum from instructions module
use crate::instruction::ContractInstructions;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// variable to hold the value of counter
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the smart contract was loaded into
    accounts: &[AccountInfo], // The account we will be interacting with
    instruction_data: &[u8], // Instructions that are passed from the client
) -> ProgramResult {
    msg!("Hello World Solana program entrypoint");
    msg!("Instruction data: {:?}", instruction_data);
    // create an instance of the unpack method from contract instruction
    let instruction = ContractInstructions::unpack(instruction_data)?;
    msg!("Instruction: {:?}", instruction);

    // Create an iterator from  the accounts array slice
    let accounts_iter = &mut accounts.iter();

    // Get the account to interact with
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    // This particular contract can run 3 operations, increment or decrement the counter, and set it to a specified value
    match instruction {
        ContractInstructions::Increment => {
            greeting_account.counter += 1;
        }
        ContractInstructions::Decrement => {
            greeting_account.counter -= 1;
        }
        ContractInstructions::Set(val) => {
            greeting_account.counter = val;
        }
    }
    // serialize the data and send it back to the client
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    // log message to check that the counter has been updated
    msg!("the counter is now {}", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    // This first test makes sure that instructions sent over the network are consumed appropriately
    fn test_sanity() {
        // create variables that are used by AccountInfo struct
        // program id of the account
        let program_id = Pubkey::default();
        // public key of the account
        let key = Pubkey::default();
        // lamports (subunit of SOL)
        let mut lamports = 0;
        // data the account holds
        let mut data = vec![0; mem::size_of::<u32>()];
        // program that owns the account
        let owner = Pubkey::default();
        // initialize the acocunt
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        // create a u32 array that converts the expected value(256) to little endian bytes
        let arr = u32::to_le_bytes(256);
        /* create an array that mimics the format of the instruction data (hence size is 5)
         * this test checks if the instruction is to set a value hence it is populated with 2
         */
        let mut instruction_data = [2; 5];
        // loop through the instruction data and change its values to make sure it matches the little endian representation of 256
        for i in 0..4 {
            // we're skipping the first index because it is reserved for the instruction
            instruction_data[i + 1] = arr[i]
        }
        // create a vector from account variable to make it iterable
        let accounts = vec![account];

        // this test checks if the initial value of the count is zero, as expected
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        //next we run process instruction to set 
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            256
        );
        let instruction_data = [0; 5];

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            257
        );
    }
    #[test]
    #[should_panic]
    fn test_crash() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let arr = u32::to_le_bytes(256);
        let mut instruction_data = [1; 5];
        for i in 0..4 {
            instruction_data[i + 1] = arr[i]
        }

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    }
}
