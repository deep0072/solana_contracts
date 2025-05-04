use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult,entrypoint, pubkey::Pubkey};


entrypoint!(process_instruction);



#[derive(BorshDeserialize,BorshSerialize)]
struct OnchainData {
    count:u32
}

fn process_instruction(
    // id of this program or contract that we are  going to create
    program_id: &Pubkey,

    // list of accounts that going to be interact with this contract or account that we are deploying
    accounts: &[AccountInfo],

    // array of instruction that going to be shoved in this account or smartcontact
    instructions: &[u8]




)->ProgramResult{

    // parse data  accounts
    let mut iter = accounts.iter();
    let data_account  = next_account_info(&mut iter)?;

    // get data
    let data  = &data_account.data;

    // convert data into bytes 
    let mut  counter = OnchainData::try_from_slice(&data.borrow_mut())?;
    if counter.count == 0 {
        counter.count = 1;
    }
    else {
        counter.count = counter.count*2;
    }

    counter.serialize(&mut *data.borrow_mut());
    Ok(())

  


}


// entrypoint!(new_instruction);
// #[derive(BorshDeserialize,BorshSerialize)]
// struct OnChainData {
//     count:u32

// }

// fn new_instruction (
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8]

// ) ->ProgramResult{
//     let mut iter = accounts.iter();
//     let data_account = next_account_info(&mut iter)?;

   
    
//     let data = data_account.data;
//     let mut counter = OnChainData::try_from_slice(&data.borrow_mut())?;
//     if counter.count == 0{
//         counter.count =1;
//     }else {
//         counter.count = counter.count*2;
//     }

//     counter.serialize(&mut *data.borrow_mut());


//     Ok(())
// }