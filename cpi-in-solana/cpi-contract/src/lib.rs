use solana_progtram::{pubkey::PubKey,account_info::{next_account_info,AccountInfo}, entrypoint::ProgramResult, entrypoint};

entrypoint!(program_instruction);

fn program_instruction(
    publicKey: &PubKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) ->ProgramResult{
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?;
    let double_contract_address = next_account_info(&mut iter)?;
    // invoke another contract

    let insructions = Instruction {
        program_id:*double_contract_address.key,
        accounts: vec![AccountMeta{
            is_signer:true,
            is_writable:true,
            pubkey:*data_account.key
        }]
    };


    invoke(&instruction_data, &[data_account.clone()]);


    

    Ok(())

 }