use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct LotteryDetails {
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

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct TicketDetails {
    pub player: Pubkey,
    pub referral_wallet: Pubkey,
    pub ticket_number_arr: [u8; 6]
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct WithdrawRequest {
    pub amount: u64,
}

// #[derive(BorshDeserialize, BorshSerialize, Debug)]
//  pub struct MailAccount {
//   pub inbox: Vec<Mail>,
//   pub sent: Vec<Mail>,
// }



#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct MarketDetails {
    pub admin: Pubkey,
    pub market_pair: String,
    pub last_price: u64,
    pub upper_floor_limit: u64,
    pub lower_floor_limit: u64,
    pub market_status: u64,
    pub markey_apy: u64,
    pub options_count: u64,
    pub amount_in_pool: u64
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct OptionsBetDetails {
    pub player: Pubkey,
    pub options_market: Pubkey,
    pub referral_wallet: Pubkey,
    pub options_bet: u64,
    pub options_strike: u64,
    pub options_spread: u64,
    pub options_bet_time: u64,
    pub options_duration: u64,
    pub options_bet_amount: u64
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct ResultDetails {
    pub player: Pubkey,
    pub options_market: Pubkey,
    pub final_price: String,
    pub result_status: String
}

