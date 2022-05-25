// import the programError struct
use solana_program::program_error::ProgramError;
// import the TryInto function
use std::convert::TryInto;
// attribute that lets us print the enum values
#[derive(Debug)]
pub enum ContractInstructions {
    Increment,
    Decrement,
    Set(u32),
}
// implement enum functions
impl ContractInstructions {
    // function to unpack instructions byte array
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        /* deconstruct the instruction data into two variables
         * the first value, tag, represents the instruction we want to give the contract
         * remember there are 3 possible instructions. 0 to increment, 1 to decrement, and 2 to set the value to something
         * The rest of values are saved in the rest variable
         */
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        // now  we have to check the tag to know what instruction to pass
        match tag {
            // if its o we know we have to increment
            0 => return Ok(ContractInstructions::Increment),
            // if its 1 we know we have to decrement
            1 => return Ok(ContractInstructions::Decrement),
            // if its 2 we know we're setting a value
            2 => {
                // so we have to check what the rest of the byte array is saying
                /* since our byte array will contain 5 items and we know the first byte represents our tag,
                    that means the rest of the array will contain whatever we're setting the value to,
                     therefore it must have length of 4
                */
                if rest.len() != 4 {
                    // we return an invalid instruction data errror because the size of the array doesn't fit what we expect
                    return Err(ProgramError::InvalidInstructionData);
                }
                //  if its actually equal to 4 then we have to convert the array slice into an array with tryinto
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                // next we retrieve the value of val and then convert from bytes to actual number
                match val {
                    Ok(i) => return Ok(ContractInstructions::Set(u32::from_le_bytes(i))),
                    // if it doesn't look like the instruction we expect we return a program error
                    _ => return Err(ProgramError::InvalidInstructionData),
                }
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
