use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub authority: Pubkey,
    pub name: String,
    pub bump: u8,
    pub token_mint: Pubkey,
    pub royalties: Vec<u16>,
    pub community_wallets: Vec<Pubkey>,
    pub commission_wallet: Pubkey,
    pub commission_fee: u16,
    pub community_balances: Vec<u64>,
    pub community_pending_balances: Vec<u64>,
    pub main_balance: u64,
    pub total_round: u64,
    pub win_percents: [u16; 6],
}
impl Game
{
    pub const LEN: usize =  std::mem::size_of::<Game>();
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub bump: u8,
    pub key: Pubkey,
    pub earned_money: u64,
    pub win_times: u64,
    pub round: u64,
    pub rand: u32,
    pub played_time: u64,
    pub bet_number: u8,
    pub bet_amount: u8,
    pub reward_amount: u64,
}
impl Player
{
    pub const LEN: usize =  std::mem::size_of::<Player>();
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized wallet cannot create game")]
    UnauthorizedWallet,
    #[msg("You should bet at least 0.05 sol")]
    MinimumPrice,
    #[msg("Invalid Instruction Added")]
    InvalidInstructionAdded,
    #[msg("Invalid Program")]
    InvalidProgramId,
}