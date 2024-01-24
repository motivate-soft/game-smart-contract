use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount, Token},
    associated_token::AssociatedToken,
};
use mpl_token_metadata::pda::{PREFIX, EDITION};
use crate::state::*;

#[derive(Accounts)]
pub struct CreateGift<'info> 
{
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub target: SystemAccount<'info>,

    #[account(mut)]
    pub nft_mint: Box<Account<'info, Mint>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump,
        seeds::program = mpl_token_metadata::ID
    )]
    pub metadata: AccountInfo<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"gift".as_ref(),
            nft_mint.key().as_ref(),
        ],
        space = Gift::LEN + 8,
        bump
    )]
    pub gift: Box<Account<'info, Gift>>,

    #[account(mut)]
    pub spl_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = creator,
    )]
    pub creator_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = spl_token_mint,
        associated_token::authority = gift,
    )]
    pub gift_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = target,
    )]
    pub target_nft_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Redeem<'info>
{
    #[account(mut)]
    pub target: Signer<'info>,

    #[account(mut, address = gift.nft_mint)]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(  
        mut,    
        associated_token::mint = nft_mint,
        associated_token::authority = target
    )]  
    pub target_nft_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            b"gift".as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump = gift.bump
    )]
    pub gift: Box<Account<'info, Gift>>,

    #[account(
        associated_token::mint = gift.gate_token_mint,
        associated_token::authority = target,
    )]
    pub gate_token_ata: Box<Account<'info, TokenAccount>>,

    pub gate_nft_mint: Box<Account<'info, Mint>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            gate_nft_mint.key().as_ref(),
        ],
        bump,
        seeds::program = mpl_token_metadata::ID
    )]
    pub gate_nft_metadata: AccountInfo<'info>,

    #[account(
        associated_token::mint = gate_nft_mint,
        associated_token::authority = target
    )]
    pub gate_nft_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = gift.spl_token_mint)]
    pub spl_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = gift,
    )]
    pub gift_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = target,
        associated_token::mint = spl_token_mint,
        associated_token::authority = target,
    )]
    pub target_token_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
 
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BurnGift<'info>
{
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub target: SystemAccount<'info>,

    #[account(mut, address = gift.nft_mint)]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            b"gift".as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump = gift.bump
    )]
    pub gift: Box<Account<'info, Gift>>,


    #[account(mut, address = gift.spl_token_mint)]
    pub spl_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = gift,
    )]
    pub gift_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = target,
        associated_token::mint = spl_token_mint,
        associated_token::authority = target,
    )]
    pub target_token_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
 
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClosePda<'info> {
    #[account(mut, address = "3qWq2ehELrVJrTg2JKKERm67cN6vYjm1EyhCEzfQ6jMd".parse::<Pubkey>().unwrap())]
    pub signer: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}