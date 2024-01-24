mod ins;
mod state;
mod nft_vault;
mod constants;

use anchor_lang::prelude::*;

use ins::*;

declare_id!("BPpeLp15ArmbtZMTmf8fSr9mQYPtpy7aVxaSCcjYpKSq");

#[program]
pub mod mint_machine {
    use super::*;

    pub fn initialize_nft_vault(ctx: Context<InitializeNftVault>, pool_bump: u8, mint_price: u64, total_supply: u32, name: String, symbol: String, creator: Pubkey) -> Result<()>
    {
        nft_vault::initialize_nft_vault(ctx, pool_bump, mint_price, total_supply, name, symbol, creator)
    }

    pub fn set_mint_price(ctx: Context<SetMintPrice>, mint_price: u64) -> Result<()>
    {
        nft_vault::set_mint_price(ctx, mint_price)
    }

    pub fn buy_from_vault(ctx: Context<BuyFromVault>) -> Result<()>
    {
        nft_vault::buy_from_vault(ctx)
    }

    pub fn add_uri(ctx: Context<AddUri>, uri: Vec<u8>) -> Result<()> 
    {
        nft_vault::add_uri(ctx, uri)
    }

    pub fn mint(ctx: Context<MintNft>) -> Result<()>
    {
        nft_vault::mint(ctx)
    }
}

