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
    pub main_balance: u64,
    pub community_balances: Vec<u64>,
    pub community_pending_balances: Vec<u64>,
    pub jackpot: u64,
    pub win_percents: Vec<Vec<u16>>,
    pub bet_prices: Vec<u64>,
    pub min_rounds_before_win: u8,
    pub lose_counter: u8,
}
impl Game
{
    pub const LEN: usize =  std::mem::size_of::<Game>() + 50 * 5 + 18 * 10;
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub earned_money: u64,
    pub key: Pubkey,
    pub status: u32,
    pub is_jackpot: bool,
    pub equal_no: u32,
    pub equal_count: u32,
    pub multipler: u32,
    pub bump: u8,
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
    InvalidProgramId
}