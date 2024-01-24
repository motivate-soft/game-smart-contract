use anchor_lang::prelude::*;

#[account]
pub struct Gift 
{
    pub creator: Pubkey,

    pub spl_token_mint: Pubkey,

    pub gate_token_mint: Pubkey,

    pub gate_token_amount: u64,

    pub verified_creators: Vec<Pubkey>,

    pub destination_address: Pubkey,

    pub token_amount: u64,
    
    pub nft_mint: Pubkey,

    pub expiration_time: u64,

    pub created_time: u64,

    pub redeemed_time: u64,

    pub redeemed: bool,

    pub burned: bool,

    pub bump: u8,
}

impl Gift {
    pub const LEN: usize = std::mem::size_of::<Gift>() + 32 * 10;
}

#[error_code]
pub enum ErrorCode
{
    #[msg("Mint Failed")]
    MintFailed,
    #[msg("Metadata Create Failed")]
    MetadataCreateFailed,
    #[msg("Already Redeemed")]
    AlreadyRedeemed,
    #[msg("Time Expired")]
    Expired,
    #[msg("Time Not Expired")]
    NotExpired,
    #[msg("Invalid Gate Token Holder")]
    InvalidHolder,
    #[msg("Expire time should be at least 8hrs")]
    ExpireMinTime,
}