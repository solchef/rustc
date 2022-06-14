use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LotteryDetails {
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TicketDetails {
    pub player: String,
    pub ticket_count: u64,
    pub ticket_number_arr: [u8; 128],
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct WithdrawRequest {
    pub amount: u64,
}
