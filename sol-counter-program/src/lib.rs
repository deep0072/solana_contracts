// process to access the data of pda 

// iterate account 
// then get data .data notation and desrializer into Struct
// using patern matching play with data
// then serialize the data 








use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,entrypoint,
    pubkey::Pubkey,
    msg,

};

#[derive(BorshDeserialize,BorshSerialize)]

enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshDeserialize,BorshSerialize)]
struct  Counter{
    count:u32
}

entrypoint!(counter_contract);
fn counter_contract(program_id:&Pubkey, accounts:&[AccountInfo],instruction_data:&[u8])->ProgramResult{

    // get all accounts
    let acc = next_account_info(&mut accounts.iter())?;
    // read all instructions
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    // deserialize the data into struct that available in account data model
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    // based on instruction perform logic
    match instruction_type{
        InstructionType::Increment(value)=>{
            msg!("Incrementing....");
            counter_data.count +=value;
        },
        InstructionType::Decrement(value)=>{

            msg!("Decrementing....");
            counter_data.count-=value;
        },
        
    }

    // then write modified data back to the account
    counter_data.serialize(&mut *acc.data.borrow_mut());

    // acc.data.borrow_mut() =>  returns a RefMut<i32>

    // then get actual value by derefernces "*"" *acc.data.borrow_mut()
    // then after getting value used "&mut" means refrence is mutable

    Ok(())



    



}