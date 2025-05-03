use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::{self,ProgramResult}, pubkey::Pubkey};


entrypoint!(process_instruction);

fn process_instruction(
    // id of this program or contract that we are  going to create
    program_id: &pubkey,

    // list of accounts that going to be interact with this contract or account that we are deploying
    accounts: &[AccountInfo],

    // array of instruction that going to be shoved in this account or smartcontact
    instructions: &[u8]




)->ProgramResult{

    // parse data  accounts
    let mut iter = accounts.iter();
    let data_account  = next_account_info(&mut iter)?;



}