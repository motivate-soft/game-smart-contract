use anchor_lang::{
  prelude::*,
  system_program,
  solana_program::program::invoke,
};
use anchor_spl::token::{transfer, Transfer, MintTo, mint_to};
use mpl_token_metadata::instruction::{create_metadata_accounts_v2};
use std::str::from_utf8;
use crate::ins::*;
use crate::constants::*;
use crate::state::{ErrorCode};

pub fn initialize_nft_vault(ctx: Context<InitializeNftVault>, pool_bump: u8, mint_price: u64, total_supply: u32, name: String, symbol: String, creator: Pubkey) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;
  nft_vault.authority = ctx.accounts.authority.key();
  nft_vault.pool_bump = pool_bump;
  nft_vault.mint_price = mint_price;
  nft_vault.total_supply = total_supply;
  nft_vault.sold_mints = vec![];
  nft_vault.uris = vec![];
  nft_vault.name = name;
  nft_vault.symbol = symbol;
  nft_vault.creator = creator;
  
  msg!("Mint Price: {:?}", mint_price);
  msg!("Total Supply: {:?}", total_supply);

  Ok(())
}

pub fn set_mint_price(ctx: Context<SetMintPrice>, mint_price: u64) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;
  nft_vault.mint_price = mint_price;

  msg!("Mint Price: {:?}", mint_price);

  Ok(())
}

pub fn buy_from_vault(ctx: Context<BuyFromVault>) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;

  if nft_vault.sold_mints.iter().any(|x| x == &ctx.accounts.nft_mint.key()) {
    return Err(ErrorCode::AlreadyMinted.into());
  }

  if nft_vault.total_supply as usize == nft_vault.sold_mints.len()  {
    return Err(ErrorCode::NotEnoughTokens.into());
  }

  if ctx.accounts.buyer.lamports() < nft_vault.mint_price {
    return Err(ErrorCode::NotEnoughSol.into())
  }

  system_program::transfer(
    CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info().clone(),
            to: ctx.accounts.nft_vault_pool.to_account_info(),
        },
    ),
    nft_vault.mint_price,
  )?;

  let vault_address = nft_vault.key().clone();
  let seeds = [
    NFT_VAULT_POOL_SEED.as_bytes(),
    vault_address.as_ref(),
    &[nft_vault.pool_bump]
  ];

  let cpi_context = CpiContext::new(
    ctx.accounts.token_program.to_account_info().clone(),
    Transfer {
      from: ctx.accounts.vault_pool_ata.to_account_info().clone(),
      to: ctx.accounts.buyer_ata.to_account_info().clone(),
      authority: ctx.accounts.nft_vault_pool.to_account_info().clone(),
    }
  );
  transfer(cpi_context.with_signer(&[&seeds[..]]), 1)?;

  nft_vault.sold_mints.push(ctx.accounts.nft_mint.key());
  
  msg!("Buyer: {:?}", ctx.accounts.buyer.key());
  msg!("NFT Mint: {:?}", ctx.accounts.nft_mint.key());
  msg!("Minted NFTS: {:?}", nft_vault.sold_mints.len());

  Ok(())
}

pub fn add_uri(ctx: Context<AddUri>, uri: Vec<u8>) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;
  
  let hash = from_utf8(&uri)
    .map_err(|err| {
      msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
      ProgramError::InvalidInstructionData
    })?;
  msg!("Uri: {:?}", hash);

  nft_vault.uris.push(uri);

  Ok(())
}

pub fn mint(ctx: Context<MintNft>) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;

  if nft_vault.total_supply as usize == nft_vault.sold_mints.len()  {
    return Err(ErrorCode::NotEnoughTokens.into());
  }

  if ctx.accounts.payer.lamports() < nft_vault.mint_price {
    return Err(ErrorCode::NotEnoughSol.into())
  }
  
  system_program::transfer(
    CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.payer.to_account_info().clone(),
            to: ctx.accounts.nft_vault_pool.to_account_info(),
        },
    ),
    nft_vault.mint_price,
  )?;

  let cpi_context = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    MintTo {
      mint: ctx.accounts.mint.to_account_info(),
      to: ctx.accounts.token_account.to_account_info(),
      authority: ctx.accounts.payer.to_account_info(),
    }
  );
  let result =  mint_to(cpi_context, 1);
  if let Err(_) = result {
    return Err(ErrorCode::MintFailed.into());
  }
  msg!("Token Minted!");

  msg!("Metadata account creating:");
  let accounts = vec![
    ctx.accounts.token_metadata_program.to_account_info(),
    ctx.accounts.metadata.to_account_info(),
    ctx.accounts.mint.to_account_info(),
    ctx.accounts.mint_authority.to_account_info(),
    ctx.accounts.payer.to_account_info(),
    ctx.accounts.rent.to_account_info(),
    ctx.accounts.token_program.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
  ];
  let creators = vec![
    mpl_token_metadata::state::Creator {
      address: nft_vault.creator,
      verified: false,
      share: 0
    },
    mpl_token_metadata::state::Creator {
      address: ctx.accounts.mint_authority.key(),
      verified: false,
      share: 100
    }
  ];

  let name = &nft_vault.name;
  let symbol = &nft_vault.symbol;
  let len = nft_vault.sold_mints.len();
  let uri = from_utf8(&nft_vault.uris[len])
    .map_err(|err| {
      msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
      ProgramError::InvalidInstructionData
    })?;
  let result = invoke(
    &create_metadata_accounts_v2(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.mint_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        name.to_string() + " #" + &(len + 1).to_string(),
        symbol.to_string(),
        "https://arweave.net/".to_owned() + &uri.to_string(),
        Some(creators),
        1,
        true,
        false,
        None,
        None,
    ),
    &accounts
);
if let Err(_) = result {
    return Err(ErrorCode::MetadataCreateFailed.into());
}
msg!("Metadata account created !!!");

nft_vault.sold_mints.push(ctx.accounts.mint.key());

Ok(())
}