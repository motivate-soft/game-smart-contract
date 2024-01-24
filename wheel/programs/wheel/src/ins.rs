use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use anchor_spl::token::{Token};

use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGame<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  
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
pub struct FundPrize<'info> {
  #[account(mut)]
  pub funder: Signer<'info>,

  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref()
    ],
    bump = game.bump
  )]
  pub game: Account<'info, Game>,

  #[account(
    mut,
    constraint = game_ata.mint == prize_mint.key() && game_ata.owner == game.key()
  )]
  pub game_ata: Account<'info, TokenAccount>,

  #[account(
    mut,
    constraint = funder_ata.mint == prize_mint.key() && funder_ata.owner == funder.key()
  )]
  pub funder_ata: Account<'info, TokenAccount>,

  pub prize_mint: Account<'info, Mint>,

  pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawPrize<'info> {
  #[account(mut)]
  pub withdrawer: Signer<'info>,

  #[account(
    mut,
    constraint = withdrawer_ata.owner == withdrawer.key() && withdrawer_ata.mint == prize_mint.key()
  )]
  pub withdrawer_ata: Account<'info, TokenAccount>,

  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref()
    ],
    bump = game.bump
  )]
  pub game: Account<'info, Game>,

  #[account(
    mut,
    constraint = game_ata.owner == game.key() && game_ata.mint == prize_mint.key()
  )]
  pub game_ata: Account<'info, TokenAccount>,

  pub prize_mint: Account<'info, Mint>,

  pub token_program: Program<'info, Token>,
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
    constraint = game_treasury_ata.mint == game.spl_mint && game_treasury_ata.owner == game.key()
  )]
  pub game_treasury_ata: Account<'info, TokenAccount>,

  #[account(
    mut,
    constraint = game.community_wallets.iter().any(|x| x == &community_wallet.key())
  )]
  pub community_wallet: SystemAccount<'info>,
  
  #[account(
    mut,
    constraint = community_treasury_ata.mint == game.spl_mint && community_treasury_ata.owner == community_wallet.key()
  )]
  pub community_treasury_ata: Account<'info, TokenAccount>,

  pub token_program: Program<'info, Token>,  
}

#[derive(Accounts)]
pub struct Play<'info>
{
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    mut,
    constraint = payer_ata.mint == game.spl_mint && payer_ata.owner == payer.key()
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
    constraint = game_treasury_ata.mint == game.spl_mint && game_treasury_ata.owner == game.key()
  )]
  pub game_treasury_ata: Account<'info, TokenAccount>,

  #[account(
    mut,
    address = game.commission_wallet
  )]
  pub commission_treasury: SystemAccount<'info>,
  
  #[account(
    mut,
    constraint = commission_treasury_ata.mint == game.spl_mint && commission_treasury_ata.owner == commission_treasury.key()
  )]
  pub commission_treasury_ata: Account<'info, TokenAccount>,

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

  pub token_program: Program<'info, Token>,
}