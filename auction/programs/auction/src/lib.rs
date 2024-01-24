mod ins;
mod state;
mod constants;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use anchor_spl::token::{transfer, Transfer};

use crate::constants::*;
use crate::state::*;
use crate::ins::*;

declare_id!("3AQ3XWGLWcWQ8BFf8VHKV2pz6YSimtAjaSbvHgZCsV5X");

#[program]
pub mod auction {
    use super::*;

    pub fn create_auction(
        ctx: Context<CreateAuction>, 
        auction_name: String, 
        min_bid_price: u64, 
        auction_duration: u64,
        spl_token_mint: Pubkey,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        
        auction.name = auction_name;
        auction.creator = ctx.accounts.creator.key();
        auction.nft_mint = ctx.accounts.nft_mint.key();
        auction.spl_token_mint = spl_token_mint;
        auction.min_bid_price = min_bid_price;
        let now: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        auction.auction_started_time = now;
        auction.auction_finish_time = now.checked_add(auction_duration).unwrap();
        auction.last_bidder = auction.creator;
        auction.transfered_to_winner = false;
        auction.bump = *ctx.bumps.get(AUCTION_SEED).unwrap();

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.creator.to_account_info(),
                from: ctx.accounts.creator_nft_ata.to_account_info(),
                to: ctx.accounts.auction_nft_ata.to_account_info(),
            },
        );

        transfer(cpi_context, 1)?;
        Ok(())
    }

    pub fn update_auction(
        ctx: Context<UpdateAuction>,
        auction_duration: u64,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.auction_finish_time = auction.auction_started_time.checked_add(auction_duration).unwrap();

        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, bid_price: u64) -> Result<()> {
        let auction = &ctx.accounts.auction;
        require!(bid_price > auction.min_bid_price, CustomError::MinBidPrice);

        let now: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        require!(now < auction.auction_finish_time, CustomError::AuctionFinished);

        let last_bid_price = auction.min_bid_price;
        let name = &auction.name;
        let creator = auction.creator;
        let bump = auction.bump;
        let seeds = &[
            AUCTION_SEED.as_ref(),
            name.as_ref(),
            creator.as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]]; 

        if auction.last_bidder != auction.creator {
            let cpi_context = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.auction.to_account_info(),
                    from: ctx.accounts.auction_token_ata.to_account_info(),
                    to: ctx.accounts.last_bidder_ata.to_account_info(),
                },
                signer,
            );
    
            transfer(cpi_context, last_bid_price)?;
        }

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.bidder.to_account_info(),
                from: ctx.accounts.bidder_ata.to_account_info(),
                to: ctx.accounts.auction_token_ata.to_account_info(),
            }
        );

        transfer(cpi_context, bid_price)?;

        let auction = &mut ctx.accounts.auction;

        
        auction.min_bid_price = bid_price;
        auction.last_bidder = ctx.accounts.bidder.key();

        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.signer))]
    pub fn transfer_to_winner(ctx: Context<TransferToWinner>) -> Result<()> {
        let auction = &ctx.accounts.auction;
        let now: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        require!(now > auction.auction_finish_time, CustomError::AuctionNotFinished);

        let name = &auction.name;
        let creator = auction.creator;
        let bump = auction.bump;
        let seeds = &[
            AUCTION_SEED.as_ref(),
            name.as_ref(),
            creator.as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]]; 

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.auction.to_account_info(),
                from: ctx.accounts.auction_nft_ata.to_account_info(),
                to: ctx.accounts.winner_nft_ata.to_account_info(),
            },
            signer,
        );

        transfer(cpi_context, 1)?;

        let auction = &mut ctx.accounts.auction;
        auction.transfered_to_winner = true;
        Ok(())
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>) -> Result<()> {
        let auction = &ctx.accounts.auction;

        require!(auction.transfered_to_winner, CustomError::TransferedToWinner);

        let name = &auction.name;
        let creator = auction.creator;
        let bump = auction.bump;
        let seeds = &[
            AUCTION_SEED.as_ref(),
            name.as_ref(),
            creator.as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]]; 

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.auction.to_account_info(),
                from: ctx.accounts.auction_token_ata.to_account_info(),
                to: ctx.accounts.creator_token_ata.to_account_info(),
            },
            signer,
        );

        transfer(cpi_context, ctx.accounts.auction_token_ata.amount)?;

        Ok(())
    }
}

pub fn authorized_admin(signer: &AccountInfo) -> Result<()> {
    let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == signer.key());
    if index == false {
        return Err(CustomError::UnauthorizedWallet.into());
    }
    Ok(())
}