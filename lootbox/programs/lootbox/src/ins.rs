use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::{
  token::{Mint, TokenAccount, Token},
  associated_token::AssociatedToken,
};

use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateLootbox<'info> 
{
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      init, 
      payer = authority, 
      space = Lootbox::LEN + 8,
      seeds = [
        GAME_SEED_PREFIX.as_bytes(),
        name.as_bytes(),
        authority.key().as_ref()
      ],
      bump
    )]
    pub lootbox: Box<Account<'info, Lootbox>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfigLootbox<'info> {
  #[account(mut, address = lootbox.authority)]
  pub authority: Signer<'info>,

  #[account(
    mut,
    seeds = [
      GAME_SEED_PREFIX.as_bytes(),
      lootbox.name.as_bytes(),
      authority.key().as_ref(),
    ],
    bump = lootbox.bump,
  )]
  pub lootbox: Box<Account<'info, Lootbox>>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  
  #[account(
    init,
    payer = payer,
    space = Player::LEN + 8,
    seeds = [
      PLAYER_SEED_PREFIX.as_bytes(),
      payer.key().as_ref(),
    ],
    bump
  )]
  pub player: Box<Account<'info, Player>>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    mut,
    seeds = [
      GAME_SEED_PREFIX.as_bytes(),
      lootbox.name.as_bytes(),
      lootbox.authority.as_ref(),
    ],
    bump = lootbox.bump,
  )]
  pub lootbox: Account<'info, Lootbox>,

  #[account(address = lootbox.balance.token_mint)]
  pub token_mint: Box<Account<'info, Mint>>,

  #[account(
    mut,
    associated_token::authority = payer,
    associated_token::mint = token_mint,
  )]
  pub payer_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    init_if_needed,
    payer = payer,
    associated_token::authority = lootbox,
    associated_token::mint = token_mint,
  )]
  pub lootbox_ata: Box<Account<'info, TokenAccount>>,

  pub system_program: Program<'info, System>,

  pub token_program: Program<'info, Token>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Withdraw<'info>
{
    #[account(mut, address = lootbox.authority)]
    pub claimer: Signer<'info>,

    #[account(
      mut,
      seeds = [
        GAME_SEED_PREFIX.as_bytes(),
        lootbox.name.as_bytes(),
        lootbox.authority.as_ref(),
      ],
      bump = lootbox.bump,
    )]
    pub lootbox: Account<'info, Lootbox>,

    #[account(
      mut,
      associated_token::authority = claimer,
      associated_token::mint = lootbox.balance.token_mint,
    )]
    pub claimer_ata: Box<Account<'info, TokenAccount>>,

    #[account(
      mut,
      associated_token::authority = lootbox,
      associated_token::mint = lootbox.balance.token_mint,
    )]
    pub lootbox_ata: Box<Account<'info, TokenAccount>>,
    
    pub token_program: Program<'info, Token>,
}


#[derive(Accounts)]
pub struct Play<'info>
{
  #[account(mut, address = player.key)]
  pub payer: Signer<'info>,

  #[account(
    mut,
    seeds = [
      GAME_SEED_PREFIX.as_bytes(),
      lootbox.name.as_bytes(),
      lootbox.authority.as_ref(),
    ],
    bump = lootbox.bump,
  )]
  pub lootbox: Box<Account<'info, Lootbox>>,

  #[account(
    mut,
    seeds=[
        PLAYER_SEED_PREFIX.as_bytes(),
        payer.key().as_ref(),
    ],
    bump = player.bump
  )]
  pub player: Box<Account<'info, Player>>,

  #[account(address = lootbox.balance.token_mint)]
  pub token_mint: Box<Account<'info, Mint>>,

  #[account(
    mut,
    associated_token::authority = payer,
    associated_token::mint = token_mint,
  )]
  pub payer_ata: Box<Account<'info, TokenAccount>>,

  
  #[account(
    mut,
    associated_token::authority = lootbox,
    associated_token::mint = token_mint,
  )]
  pub lootbox_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    init_if_needed,
    payer = payer,
    associated_token::authority = player,
    associated_token::mint = token_mint,
  )]
  pub player_ata: Box<Account<'info, TokenAccount>>,

  pub system_program: Program<'info, System>,

  pub token_program: Program<'info, Token>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub rent: Sysvar<'info, Rent>,

  /// CHECK: instruction_sysvar_account cross checking 
  #[account(address = sysvar::instructions::ID)]
  pub instruction_sysvar_account: AccountInfo<'info>,
  
  /// CHECK:
  #[account(address = sysvar::slot_hashes::id())]
  pub recent_slothashes: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Claim<'info>
{
    #[account(mut, address = player.key)]
    pub claimer: Signer<'info>,

    #[account(
      mut,
      seeds=[
          PLAYER_SEED_PREFIX.as_bytes(),
          claimer.key().as_ref(),
      ],
      bump = player.bump
    )]
    pub player: Box<Account<'info, Player>>,
  
    #[account(constraint = player.balances.iter().any(|x| x.token_mint == token_mint.key()))]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
      mut,
      associated_token::authority = claimer,
      associated_token::mint = token_mint,
    )]
    pub claimer_ata: Account<'info, TokenAccount>,

    #[account(
      mut,
      associated_token::authority = player,
      associated_token::mint = token_mint,
    )]
    pub player_ata: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    /// CHECK: instruction_sysvar_account cross checking 
    #[account(address = sysvar::instructions::ID)]
    pub instruction_sysvar_account: AccountInfo<'info>,
}