use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    if instruction_data[0] == 0 {
        return create_lottery(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return play(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );

    }

    else if instruction_data[0] == 5 {
        return create_market(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );

    }

    else if instruction_data[0] == 6 {
        return place_option(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );

    }

    else if instruction_data[0] == 7 {
        return result_option(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );

    }
    msg!("Didn't found the entrypoint required");
    Err(ProgramError::InvalidInstructionData)
}
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct LotteryDetails {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub lottery_start: String,
    pub lottery_end: String,
    pub ticket_price: u64,
    pub amount_in_pot: u64,
    pub total_entries: u64,
}


fn create_lottery(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let creator_account = next_account_info(accounts_iter)?;
    if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut input_data = LotteryDetails::try_from_slice(&instruction_data)
        .expect("Instruction data serialization didn't worked");

    if input_data.admin != *creator_account.key {
        msg!("Invaild instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }
    let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
    if **writing_account.lamports.borrow() < rent_exemption {
        msg!("The balance of writing_account should be more then rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }
    input_data.amount_in_pot = 0;
    input_data.total_entries = 0;

    input_data.serialize(&mut &mut writing_account.try_borrow_mut_data()?[..])?;
    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct WithdrawRequest {
    pub amount: u64,
}

fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !admin_account.is_signer {
        msg!("admin should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }
    let fanilotto_data = LotteryDetails::try_from_slice(*writing_account.data.borrow())
        .expect("Error deserialaizing data");

    if fanilotto_data.admin != *admin_account.key {
        msg!("Only the account admin can withdraw");
        return Err(ProgramError::InvalidAccountData);
    }
    let input_data = WithdrawRequest::try_from_slice(&instruction_data)
        .expect("Instruction data serialization didn't worked");

    let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
    if **writing_account.lamports.borrow() - rent_exemption < input_data.amount {
        msg!("Insufficent balance");
        return Err(ProgramError::InsufficientFunds);
    }


    **writing_account.try_borrow_mut_lamports()? -= input_data.amount;
    **admin_account.try_borrow_mut_lamports()? += input_data.amount;

    Ok(())
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TicketDetails {
    pub player: Pubkey,
    pub referral_wallet: Pubkey,
    pub ticket_number_arr: [u8; 6]
}

fn play(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let player_program_account = next_account_info(accounts_iter)?;
    let player = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if player_program_account.owner != program_id {
        msg!("player_program_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !player.is_signer {
        msg!("player should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut fanilotto_data = LotteryDetails::try_from_slice(*writing_account.data.borrow())
        .expect("Error deserialaizing data");

        let mut ticket_data = TicketDetails::try_from_slice(&instruction_data)
        .expect("Error deserialaizing data");

        // validating the ticket purchased

    // for i in 0..5 {
    //     if ticket_number_arr[i] < 1 || ticket_number_arr[i] > 69 {
    //         msg!("Invalid value for one of from 1 to 5 number");
    //         return Err(LotteryError::InvalidNumber.into());
    //     }
    // }

    // if ticket_number_arr[5] < 1 || ticket_number_arr[5] > 29 {
    //     msg!("Invalid value for 6 number");
    //     return Err(LotteryError::InvalidNumber.into());
    // }

    ticket_data.ticket_number_arr = ticket_data.ticket_number_arr;

    fanilotto_data.amount_in_pot += **player_program_account.lamports.borrow();
    fanilotto_data.total_entries += 1;

    // **writing_account.try_borrow_mut_lamports()? += **player_program_account.lamports.borrow();
    // **player_program_account.try_borrow_mut_lamports()? = 0;

    fanilotto_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    ticket_data.serialize(&mut &mut player_program_account.data.borrow_mut()[..])?;

    Ok(())
}


// Binary Options

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MarketDetails {
    pub admin: Pubkey,
    pub market_pair: String,
    pub last_price: String,
    pub upper_floor_limit: String,
    pub lower_floor_limit: String,
    pub market_status: String,
    pub markey_apy: String,
    pub options_count: u64,
    pub amount_in_pool: u64
}

fn create_market(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let creator_account = next_account_info(accounts_iter)?;
    if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut input_data = MarketDetails::try_from_slice(&instruction_data)
        .expect("Instruction data serialization didn't worked");

    if input_data.admin != *creator_account.key {
        msg!("Invaild instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }
    let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
    if **writing_account.lamports.borrow() < rent_exemption {
        msg!("The balance of writing_account should be more then rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }
    input_data.options_count = 0;
    input_data.amount_in_pool = 0;

    input_data.serialize(&mut &mut writing_account.try_borrow_mut_data()?[..])?;
    Ok(())
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct OptionsBetDetails {
    pub player: Pubkey,
    pub options_market: Pubkey,
    pub referral_wallet: Pubkey,
    pub options_bet: String,
    pub options_strike: String,
    pub options_spread: String,
    pub options_bet_time: String,
    pub options_duration: u64,
    pub options_bet_amount: u64
}

fn place_option(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let player_program_account = next_account_info(accounts_iter)?;
    let player = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if player_program_account.owner != program_id {
        msg!("player_program_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !player.is_signer {
        msg!("player should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

        let mut options_market_data = MarketDetails::try_from_slice(*writing_account.data.borrow())
        .expect("Error deserialaizing data");

        let options_bet_data = OptionsBetDetails::try_from_slice(&instruction_data)
        .expect("Error deserialaizing data");


    options_market_data.amount_in_pool += **player_program_account.lamports.borrow();
    options_market_data.options_count += 1;

    // **writing_account.try_borrow_mut_lamports()? += **player_program_account.lamports.borrow();
    // **player_program_account.try_borrow_mut_lamports()? = 0;

    options_market_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    options_bet_data.serialize(&mut &mut player_program_account.data.borrow_mut()[..])?;

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct ResultDetails {
    pub player: Pubkey,
    pub options_market: Pubkey,
    pub final_price: String,
    pub result_status: String
}

fn result_option(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;


    // if writing_account.owner != program_id {
    //     msg!("writing_account isn't owned by program");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    // if player_program_account.owner != program_id {
    //     msg!("player_program_account isn't owned by program");
    //     return Err(ProgramError::IncorrectProgramId);
    // }
    
        // if !player.is_signer {
        //     msg!("player should be signer");
        //     return Err(ProgramError::IncorrectProgramId);
        // }

        //     let mut options_market_data = MarketDetails::try_from_slice(*writing_account.data.borrow())
        //     .expect("Error deserialaizing data");

        let result_option_data = ResultDetails::try_from_slice(&instruction_data)
        .expect("Error deserialaizing data");
    

        println!("Fetching Market: {:?}", result_option_data.options_market);
        
         // Latest price for resulting


        // options_market_data.amount_in_pool += **player_program_account.lamports.borrow();
        // options_market_data.options_count += 1;

        **admin_account.try_borrow_mut_lamports()? += **writing_account.lamports.borrow();
        **writing_account.try_borrow_mut_lamports()? = 0;
        
        // options_market_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
        // options_bet_data.serialize(&mut &mut player_program_account.data.borrow_mut()[..])?;

    Ok(())
}



