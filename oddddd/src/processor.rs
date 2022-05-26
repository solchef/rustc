use crate::state::TicketDetails;
use solana_program::sysvar::Sysvar;
use crate::state::LotteryDetails;
use crate::error::MailError::NotWritable;
use crate::instruction::FanitradeUtilsInstructions;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
entrypoint::ProgramResult, msg, rent::Rent, program_error::ProgramError, pubkey::Pubkey,account_info::{AccountInfo},
// program::{invoke},
};

// use spl_token::{instruction::transfer, state::Account};


pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = FanitradeUtilsInstructions::unpack(instruction_data)?;
    msg!("Failed to execute query: {}");
    match instruction {
    //   FanitradeUtilsInstructions::InitFanitradeUtils => {
    //     msg!("Instruction: Initilalize Fanitrade Utils Account");
    //     Self::process_init_fanitrade_utils(&accounts[0], program_id)
    //   }
      FanitradeUtilsInstructions::CreateLottery { lottery } => {
        msg!("Instruction: Create new lottery draw");
        Self::process_create_lottery(accounts, lottery, program_id)
      }
      FanitradeUtilsInstructions::PlayFaniLotto { ticket } => {
        msg!("Instruction: Purchase new Lotto Ticket");
        Self::process_play_fanilotto(accounts, ticket, program_id)
      }
    }
  }

//   fn process_init_fanitrade_utils(account: &AccountInfo, program_id: &Pubkey) -> ProgramResult {
//     if !account.is_writable {
//       return Err(NotWritable.into());
//     }

//     if account.owner != program_id {
//       return Err(ProgramError::IncorrectProgramId);
//     }


//     Ok(())
//   }

  fn process_create_lottery(
    accounts: &[AccountInfo],
    lottery:LotteryDetails,
    program_id: &Pubkey,
  ) -> ProgramResult {
    // let accounts_iter = &mut accounts.iter();
    let writing_account = &accounts[0];
    let creator_account = &accounts[1];
    let _fanilotto_token_mint = &accounts[2];
    let _fanilotto_rewards_pool_token_account = &accounts[3];


    if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    // msg!(&instruction_data.data_len);

    let mut input_data = lottery.clone();

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

  fn process_play_fanilotto(
    accounts: &[AccountInfo],
    ticket:TicketDetails,
    program_id: &Pubkey,

) -> ProgramResult {
    // let accounts_iter = &mut accounts.iter();
    let writing_account = &accounts[0];
    let player_program_account = &accounts[1];
    let player = &accounts[2];
    // let spl_token_account = &accounts[4];
    // let token_mint = &accounts[4];
    // let lottery_pool_token_account = &accounts[4];
    // let source_token_account_owner = &accounts[2];
    // let source_token_account = &accounts[2];


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

        let mut ticket_data = ticket.clone();
    
    

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

    // let transfer_tokens_to_vesting_account = transfer(
    //     spl_token_account.key,
    //     source_token_account.key,
    //     lottery_pool_token_account.key,
    //     source_token_account_owner.key,
    //     &[],
    //     fanilotto_data.ticket_price
    // )?;

    // invoke(
    //     &transfer_tokens_to_vesting_account,
    //     &[
    //         source_token_account.clone(),
    //         lottery_pool_token_account.clone(),
    //         spl_token_account.clone(),
    //         source_token_account_owner.clone(),
    //     ],
    // )?;

    Ok(())
}



}

