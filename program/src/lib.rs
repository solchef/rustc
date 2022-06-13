use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::invoke_signed;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer_checked;

use spl_token::instruction::transfer;

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
        return play(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 5 {
        return initialize_trax(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 6 {
        return create_market(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 7 {
        return place_option(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 8 {
        return settle_option(
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
    pub is_initialized: u64,
    pub is_ended: u64,
    pub lottery_start: String,
    pub lottery_end: String,
    pub ticket_price: u64,
    pub amount_in_pot: u64,
    pub total_entries: u64,
    pub token_mint: Pubkey,
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
        .expect("Instruction data not serialized properly");

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
    input_data.is_initialized = 1;
    input_data.is_ended = 0;

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
    let token_mint = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let lotto_ata = next_account_info(accounts_iter)?;
    let admin_ata = next_account_info(accounts_iter)?;

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

    // let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
    // if **writing_account.lamports.borrow() - rent_exemption < input_data.amount {
    //     msg!("Insufficent balance");
    //     return Err(ProgramError::InsufficientFunds);
    // }

    let winner_token_account_address =
        get_associated_token_address(writing_account.key, token_mint.key);
    let admin_account_address = get_associated_token_address(admin_account.key, token_mint.key);
    msg!("lotto ATA: {:?}", winner_token_account_address);
    msg!("Admin ATA: {:?}", &admin_account_address);

    let transfer_to_winning_player_account = transfer_checked(
        token_program.key,
        &lotto_ata.key,
        token_mint.key,
        &winner_token_account_address,
        admin_account.key,
        &[],
        input_data.amount,
        9,
    )?;

    invoke_signed(
        &transfer_to_winning_player_account,
        &[
            token_program.clone(),
            admin_ata.clone(),
            lotto_ata.clone(),
            admin_account.clone(),
        ],
        &[],
    )?;
    // **writing_account.try_borrow_mut_lamports()? -= input_data.amount;

    // **admin_account.try_borrow_mut_lamports()? += input_data.amount;

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TicketDetails {
    pub player: String,
    pub ticket_count: u64,
    pub ticket_number_arr: [u8; 128],
}

fn play(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let player_program_account = next_account_info(accounts_iter)?;
    let player = next_account_info(accounts_iter)?; //wallet of user //signer
    let player_token_account = &accounts[6];
    let token_program = &accounts[5];
    let lottery_pool_token_account = &accounts[4];

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
        .expect("Error deserialaizing lotto data");

    let mut ticket_data =
        TicketDetails::try_from_slice(&instruction_data).expect("Error deserialaizing ticket data");

    let total_amount = fanilotto_data.ticket_price;
    msg!("Ticket Purchase");

    let transfer_to_lottery_pool = transfer(
        token_program.key,
        player_token_account.key,
        lottery_pool_token_account.key,
        player.key,
        &[],
        total_amount,
    )?;

    invoke(
        &transfer_to_lottery_pool,
        &[
            player_token_account.clone(),
            lottery_pool_token_account.clone(),
            token_program.clone(),
            player.clone(),
        ],
    )?;

    msg!("{:?}", ticket_data);
    ticket_data.ticket_number_arr = ticket_data.ticket_number_arr;
    ticket_data.ticket_number_arr = ticket_data.ticket_number_arr;
    ticket_data.ticket_count = ticket_data.ticket_count;
    fanilotto_data.amount_in_pot += total_amount;
    fanilotto_data.total_entries += 1;

    **writing_account.try_borrow_mut_lamports()? += **player_program_account.lamports.borrow();
    // **player_program_account.try_borrow_mut_lamports()? = 0;

    fanilotto_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    ticket_data.serialize(&mut &mut player_program_account.try_borrow_mut_data()?[..])?;
    Ok(())
}

// Fanitrax
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TraxDetails {
    pub admin: Pubkey,
    pub is_initialized: u64,
    pub trax_pool_amount: u64,
    pub total_entries: u64,
    pub total_markets: u64,
    pub active_markets: u64,
}

fn initialize_trax(
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

    // let (escrow_pubkey, bump_seed) = Pubkey::find_program_address(&[&["fanitraxacc"]], program_id);

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut input_data = TraxDetails::try_from_slice(&instruction_data)
        .expect("Instruction data not serialized properly");

    if input_data.admin != *creator_account.key {
        msg!("Invaild instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }
    let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());
    if **writing_account.lamports.borrow() < rent_exemption {
        msg!("The balance of writing_account should be more then rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }

    input_data.is_initialized = 1;
    input_data.trax_pool_amount = 0;
    input_data.total_markets = 0;
    input_data.active_markets = 0;

    input_data.serialize(&mut &mut writing_account.try_borrow_mut_data()?[..])?;
    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MarketDetails {
    pub admin: Pubkey,
    pub trax_pub: String,
    pub market_pair: String,
    pub last_price: u64,
    pub upper_floor_limit: u64,
    pub lower_floor_limit: u64,
    pub market_status: u64,
    pub markey_apy: u64,
    pub options_count: u64,
    pub amount_in_pool: u64,
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
    pub player: String,
    pub options_market: String,
    pub options_bet: u64,
    pub options_strike: u64,
    pub options_spread: u64,
    pub options_bet_start: u64,
    pub options_bet_end: u64,
    pub options_duration: u64,
    pub options_bet_amount: u64,
    pub options_bet_result: String, //undecided Won Lost
}

fn place_option(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let trader_program_account = next_account_info(accounts_iter)?;
    let trader = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if trader_program_account.owner != program_id {
        msg!("trader_program_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !trader.is_signer {
        msg!("trader should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut options_market_data = MarketDetails::try_from_slice(*writing_account.data.borrow())
        .expect("Error deserialaizing data");

    let options_bet_data =
        OptionsBetDetails::try_from_slice(&instruction_data).expect("Error deserialaizing data");

    options_market_data.amount_in_pool += **trader_program_account.lamports.borrow();
    options_market_data.options_count += 1;

    // **writing_account.try_borrow_mut_lamports()? += **trader_program_account.lamports.borrow();
    // **trader_program_account.try_borrow_mut_lamports()? = 0;

    options_market_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    options_bet_data.serialize(&mut &mut trader_program_account.data.borrow_mut()[..])?;

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct ResultDetails {
    pub player: Pubkey,
    pub options_market: Pubkey,
    pub final_price: String,
    pub result_status: String,
}

fn settle_option(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let settler_account = next_account_info(accounts_iter)?;

    // match market.get_price("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

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

    let settle_option_data =
        ResultDetails::try_from_slice(&instruction_data).expect("Error deserialaizing data");

    println!("Fetching Market: {:?}", settle_option_data.options_market);

    // Latest price for resulting

    // options_market_data.amount_in_pool += **player_program_account.lamports.borrow();
    // options_market_data.options_count += 1;

    **settler_account.try_borrow_mut_lamports()? += **writing_account.lamports.borrow();
    **writing_account.try_borrow_mut_lamports()? = 0;

    // options_market_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    // options_bet_data.serialize(&mut &mut player_program_account.data.borrow_mut()[..])?;

    Ok(())
}
