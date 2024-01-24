use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::{Token, TokenAccount};

use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGame<'info> 
{
    #[account(mut)]
    pub payer: Signer<'info>,
    // game
    #[account(
      init, 
      payer = payer, 
      space = Game::LEN + 8,
      seeds = [
        name.as_bytes(),
        GAME_SEED_PREFIX.as_bytes(),
        payer.key().as_ref()
      ],
      bump
    )]
    pub game: Account<'info, Game>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfigGame<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
  #[account(
    init,
    payer = payer,
    space = Player::LEN + 8,
    seeds = [
      PLAYER_SEED_PREFIX.as_bytes(),
      payer.key().as_ref(),
      game.key().as_ref(),
    ],
    bump
  )]
  pub player: Account<'info, Player>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    constraint = payer_ata.owner == payer.key() && payer_ata.mint == game.token_mint
  )]
  pub payer_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
  #[account(
    mut,
    constraint = game_treasury_ata.owner == game.key() && game_treasury_ata.mint == game.token_mint
  )]
  pub game_treasury_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Play<'info>
{
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    constraint = payer_ata.owner == payer.key() && payer_ata.mint == game.token_mint
  )]
  pub payer_ata: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
  #[account(
    mut,
    constraint = game_treasury_ata.owner == game.key() && game_treasury_ata.mint == game.token_mint
  )]
  pub game_treasury_ata: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    constraint = commission_treasury_ata.owner == game.commission_wallet && commission_treasury_ata.mint == game.token_mint
  )]
  pub commission_treasury_ata: Box<Account<'info, TokenAccount>>,
  #[account(
      mut,
      seeds=[
          PLAYER_SEED_PREFIX.as_bytes(),
          payer.key().as_ref(),
          player.game.as_ref(),            
      ],
      bump = player.bump
  )]
  pub player: Account<'info, Player>,
  /// CHECK: instruction_sysvar_account cross checking 
  #[account(address = sysvar::instructions::ID)]
  pub instruction_sysvar_account: AccountInfo<'info>,
  pub token_program: Program<'info, Token>,
  /// CHECK:
  #[account(address = sysvar::slot_hashes::id())]
  pub recent_slothashes: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Claim<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(
      mut,
      constraint = claimer_ata.owner == claimer.key() && claimer_ata.mint == game.token_mint
    )]
    pub claimer_ata: Account<'info, TokenAccount>,
    #[account(
      mut,
      seeds = [
        game.name.as_bytes(),
        GAME_SEED_PREFIX.as_bytes(),
        game.authority.as_ref(),
      ],
      bump = game.bump,
    )]
    pub game: Account<'info, Game>,
    #[account(
      mut,
      constraint = game_treasury_ata.owner == game.key() && game_treasury_ata.mint == game.token_mint
    )]
    pub game_treasury_ata: Account<'info, TokenAccount>,
    #[account(
      mut,
      seeds=[
          PLAYER_SEED_PREFIX.as_bytes(),
          claimer.key().as_ref(),
          player.game.as_ref(),            
      ],
      bump = player.bump
    )]
    pub player: Account<'info, Player>,
    /// CHECK: instruction_sysvar_account cross checking 
    #[account(address = sysvar::instructions::ID)]
    pub instruction_sysvar_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SendToCommunityWallet<'info> 
{
  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
  #[account(
    mut,
    constraint = game_treasury_ata.owner == game.key() && game_treasury_ata.mint == game.token_mint
  )]
  pub game_treasury_ata: Account<'info, TokenAccount>,  
  #[account(
    mut,
    constraint = game.community_wallets.iter().any(|x| x == &community_treasury_ata.owner) && community_treasury_ata.mint == game.token_mint
  )]
  pub community_treasury_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct Withdraw<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(
      mut,
      constraint = claimer_ata.owner == claimer.key() && claimer_ata.mint ==  game.token_mint
    )]
    pub claimer_ata: Account<'info, TokenAccount>,
    #[account(
      mut,
      seeds = [
        game.name.as_bytes(),
        GAME_SEED_PREFIX.as_bytes(),
        game.authority.as_ref(),
      ],
      bump = game.bump,
    )]
    pub game: Account<'info, Game>,
    #[account(
      mut,
      constraint = game_treasury_ata.owner == game.key()
    )]
    pub game_treasury_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
