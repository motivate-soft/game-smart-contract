use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(auction_name: String)]
pub struct CreateAuction<'info>
{
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init, 
        payer = creator, 
        space = Auction::LEN + 8, 
        seeds = [
            AUCTION_SEED.as_bytes(),
            auction_name.as_bytes(),
            creator.key().as_ref(),
        ], 
        bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = creator,
    )]
    pub creator_nft_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = nft_mint,
        associated_token::authority = auction,
    )]
    pub auction_nft_ata: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateAuction<'info> 
{
    #[account(mut, address = auction.creator)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [
            AUCTION_SEED.as_bytes(),
            auction.name.as_bytes(),
            auction.creator.as_ref(),
        ], 
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,
}

#[derive(Accounts)]
pub struct Bid<'info> 
{
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [
            AUCTION_SEED.as_bytes(),
            auction.name.as_bytes(),
            auction.creator.as_ref(),
        ], 
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    #[account(mut, address = auction.spl_token_mint)]
    pub spl_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = bidder,
        associated_token::mint = spl_token_mint,
        associated_token::authority = auction,
    )]
    pub auction_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = bidder,
    )]
    pub bidder_ata: Account<'info, TokenAccount>,

    #[account(address = auction.last_bidder)]
    pub last_bidder: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = bidder,
        associated_token::mint = spl_token_mint,
        associated_token::authority = last_bidder,
    )]
    pub last_bidder_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferToWinner<'info> 
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            AUCTION_SEED.as_bytes(),
            auction.name.as_bytes(),
            auction.creator.as_ref(),
        ], 
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = auction,
    )]
    pub auction_nft_ata: Account<'info, TokenAccount>,

    #[account(mut, address = auction.nft_mint)]
    pub nft_mint: Account<'info, Mint>,

    #[account(address = auction.last_bidder)]
    pub winner: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = nft_mint,
        associated_token::authority = winner,
    )]
    pub winner_nft_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> 
{
    #[account(mut, address = auction.creator)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [
            AUCTION_SEED.as_bytes(),
            auction.name.as_bytes(),
            auction.creator.as_ref(),
        ], 
        bump = auction.bump,
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        associated_token::mint = auction.spl_token_mint,
        associated_token::authority = auction,
    )]
    pub auction_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = auction.spl_token_mint,
        associated_token::authority = creator,
    )]
    pub creator_token_ata: Box<Account<'info, TokenAccount>>,
 
    pub token_program: Program<'info, Token>,
}