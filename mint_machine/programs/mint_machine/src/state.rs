use anchor_lang::prelude::*;

#[account]
pub struct NftVault {
    pub authority: Pubkey,
    pub pool_bump: u8,
    pub mint_price: u64,
    pub total_supply: u32,
    pub sold_mints: Vec<Pubkey>,
    pub name: String,
    pub symbol: String,
    pub creator: Pubkey,
    pub uris: Vec<Vec<u8>>,
}

#[error_code]
pub enum ErrorCode
{
    #[msg("Not Enough Tokens.")] // 
    NotEnoughTokens,
    #[msg("Already Minted")] //
    AlreadyMinted,
    #[msg("Not Enough SOL")] //
    NotEnoughSol,
    #[msg("Mint Failed")]
    MintFailed,
    #[msg("Metadata Create Failed")]
    MetadataCreateFailed,
}