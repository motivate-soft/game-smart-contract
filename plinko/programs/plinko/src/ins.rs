use anchor_lang::prelude::*;
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
pub struct Withdraw<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(
      mut,
      constraint = claimer_ata.owner == claimer.key() && claimer_ata.mint ==  game_treasury_ata.mint
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

#[derive(Accounts)]
pub struct Claim<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut, address = game.backend_wallet)]
    pub backend: Signer<'info>,
    #[account(
      mut,
      constraint = claimer_ata.owner == claimer.key() && claimer_ata.mint == game_treasury_ata.mint
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

#[derive(Accounts)]
pub struct Fund<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    constraint = payer_ata.owner == payer.key() && payer_ata.mint == game_treasury_ata.mint
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
    constraint = game_treasury_ata.owner == game.key()
  )]
  pub game_treasury_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
}