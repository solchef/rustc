use crate::error::MailError::InvalidInstruction;
use crate::{state::{LotteryDetails,TicketDetails }};
use borsh::BorshDeserialize;
use solana_program::{program_error::ProgramError, msg};

#[derive(Debug, PartialEq)]
pub enum FanitradeUtilsInstructions {
    CreateLottery { lottery:LotteryDetails },
    PlayFaniLotto { ticket:TicketDetails }

}

impl FanitradeUtilsInstructions {
  /// Deserialize byte buffer into a [FanitradeUtilsInstructions](enum.FanitradeUtilsInstructions.html).
  
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    msg!("Bumping in");
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    // instruction_data[0] == 0
    msg!("Failed to execute query: {}", tag);
    Ok(match tag {
      0 => Self::CreateLottery {
             lottery: LotteryDetails::try_from_slice(&rest)?,
      },
      1 => Self::PlayFaniLotto {
        ticket: TicketDetails::try_from_slice(&rest)?,
      },
      _ => return Err(InvalidInstruction.into()),
    })
  }
}

