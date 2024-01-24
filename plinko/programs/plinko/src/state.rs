use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub authority: Pubkey,
    pub name: String,
    pub bump: u8,
    pub backend_wallet: Pubkey,
}
impl Game
{
    pub const LEN: usize =  std::mem::size_of::<Game>();
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized wallet cannot create game")]
    UnauthorizedWallet,
}