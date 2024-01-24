use anchor_lang::prelude::*;

#[account]
pub struct Lootbox {
    pub authority: Pubkey,
    pub name: String,
    pub image_url: String,
    pub price: u64,
    pub win_percent: u16,
    pub balance: Balance,
    pub bump: u8,
}

impl Lootbox
{
    pub const LEN: usize =  std::mem::size_of::<Lootbox>() + 100;
}

#[account]
pub struct Player {
    pub bump: u8,
    pub key: Pubkey,
    pub balances: Vec<Balance>,
}

impl Player
{
    pub const LEN: usize =  std::mem::size_of::<Player>() + Balance::LEN * 10;
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Balance {
    pub token_mint: Pubkey,
    pub amount: u64,
}

impl Balance {
    pub const LEN: usize = std::mem::size_of::<Balance>();
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized wallet cannot create game")]
    UnauthorizedWallet,
    #[msg("You should bet at least 0.05 sol")]
    MinimumPrice,
    #[msg("Invalid Instruction Added")]
    InvalidInstructionAdded,
    #[msg("Invalid Program")]
    InvalidProgramId,
}