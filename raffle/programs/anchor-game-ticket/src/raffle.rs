use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as TransferProgramSOL};
use anchor_spl::token::{self, Transfer as TransferSPL};
use crate::state::{Raffle, BuyEvent, Buyer};
use crate::ins::*;
use crate::state::{ErrorCode};
use crate::constants::*;
use crate::utils;

pub fn initialize_raffle(ctx: Context<InitializeRaffle>, token_spl_address: Pubkey, ticket_price: u64, amount: u32, store_buyers: bool, transfer_token: bool, nft_mint_address: Pubkey) -> Result<()>
{
    let global = &ctx.accounts.global;
    let raffle = &mut ctx.accounts.raffle;

    raffle.owner = ctx.accounts.payer.key();
    raffle.token_spl_address = token_spl_address;
    raffle.price_per_ticket = ticket_price;
    raffle.total_tickets = amount;
    raffle.sold_tickets = 0;
    raffle.store_buyers = store_buyers;
    raffle.nft_mint_address = nft_mint_address;
    raffle.buyers = vec![];

    if ctx.accounts.sender_ata.amount.clone() < 1 as u64
    {
        return err!(ErrorCode::NotEnoughTokens);
    }

    if transfer_token
    {
        // Transfer NFT to raffle bank
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferSPL {
                    from: ctx.accounts.sender_ata.to_account_info(),
                    to: ctx.accounts.raffle_pool_ata.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            1,
        )?;
    }
    else
    {
        // if user chosen not to transfer token to raffle bank,
        // we must make sure he is an authorized admin, otherwise raise an error
        if global.authorized_admins.iter().any(|x| x == &ctx.accounts.payer.key()) == false
        {
            return Err(ErrorCode::NotAuthorizedAdmin.into());
        }
    }

    msg!("Version: {:?}", VERSION);
    msg!("Program initialized successfully with Bank Vault.");
    msg!("Total Tickets: {:?}", raffle.total_tickets);
    msg!("Sold Tickets: {:?}", raffle.sold_tickets);
    msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("NFT Mint Address: {:?}", raffle.nft_mint_address);
    msg!("Token SPL Address: {:?}", raffle.token_spl_address);
    msg!("Store Buyers: {:?}", raffle.store_buyers);
    msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

    Ok(())
}

pub fn initialize_with_pda(ctx: Context<InitializeWithPDA>, pool_bump: u8, token_spl_address: Pubkey, ticket_price: u64, amount: u32, store_buyers: bool, nft_mint_address: Pubkey) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;

    raffle.pool_bump = pool_bump;
    raffle.token_spl_address = token_spl_address;
    raffle.price_per_ticket = ticket_price;
    raffle.total_tickets = amount;
    raffle.sold_tickets = 0;
    raffle.store_buyers = store_buyers;
    raffle.nft_mint_address = nft_mint_address;
    raffle.buyers = vec![];

    if ctx.accounts.sender_ata.amount.clone() < 1 as u64
    {
        return err!(ErrorCode::NotEnoughTokens);
    }

    /* Option A: */
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferSPL {
                from: ctx.accounts.sender_ata.to_account_info(),
                to: ctx.accounts.raffle_pool_ata.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Version: {:?}", VERSION);
    msg!("Program initialized successfully with PDA.");
    msg!("Total Tickets: {:?}", raffle.total_tickets);
    msg!("Sold Tickets: {:?}", raffle.sold_tickets);
    msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("NFT Mint Address: {:?}", raffle.nft_mint_address);
    msg!("Token SPL Address: {:?}", raffle.token_spl_address);
    msg!("Store Buyers: {:?}", raffle.store_buyers);
    msg!("Raffle Pool ATA: {:?}", ctx.accounts.raffle_pool_ata.to_account_info().key);
    msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

    Ok(())
}

pub fn buy_ticket_sol(ctx: Context<BuyTicketSOL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;
    let transaction_price = raffle.price_per_ticket * amount as u64;

    msg!("SOL Transfer: {:?}", raffle.token_spl_address.key());

    // transfer via SOL
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            TransferProgramSOL {
                from: ctx.accounts.buyer.to_account_info().clone(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        ),
        transaction_price,
    )?;

    msg!("Token Type: SOL");

    update_raffle(raffle, ctx.accounts.buyer.key(), amount)
}

pub fn buy_ticket_spl(ctx: Context<BuyTicketSPL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;
    let transaction_price = raffle.price_per_ticket * amount as u64;

    msg!("SPL-Token Transfer: {:?}", raffle.token_spl_address.key());

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferSPL {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.recipient_tokens.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        transaction_price,
    )?;

    msg!("Version: {:?}", VERSION);
    msg!("Token Type: {:?}", raffle.token_spl_address.key());

    update_raffle(raffle, ctx.accounts.sender.key(), amount)
}

pub fn update_raffle(raffle: &mut Raffle, buyer: Pubkey, amount: u32) -> Result<()>
{
    raffle.sold_tickets = raffle.sold_tickets.checked_add(amount).unwrap();
    let transaction_price = raffle.price_per_ticket * amount as u64;

    emit!(BuyEvent
        {
            buyer: buyer,
            amount: amount,
            sold_tickets: raffle.sold_tickets,
            total_tickets: raffle.total_tickets,
            remaining_tickets: raffle.total_tickets.checked_sub(raffle.sold_tickets).unwrap()
        });

    let remaining_tickets = raffle.total_tickets.checked_sub(raffle.sold_tickets).unwrap();

    // store the buyer if feature is enabled
    if raffle.store_buyers == true
    {
        let index = raffle.buyers.iter().position(|x| x.key == buyer);
        if let Some(index) = index
        {
            let item = &mut raffle.buyers[index];
            item.tickets = item.tickets.checked_add(amount).unwrap();
        }
        else
        {
            let item = Buyer
            {
                key: buyer,
                tickets: amount,
            };
            raffle.buyers.push(item);
        }
    }

    msg!("Version: {:?}", VERSION);
    msg!("Buyer: {:?}", buyer);
    msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

    Ok(())
}

// Withdraw tokens from PDA
pub fn withdraw_from_pda(ctx: Context<WithdrawFromPDA>, amount: u64) -> Result<()> 
{
    let global = &ctx.accounts.global;
    if global.authorized_admins.iter().any(|x| x == &ctx.accounts.admin.key()) == false {
        return Err(ErrorCode::NotAuthorizedAdmin.into());
    }

    let raffle = &ctx.accounts.raffle;
    let raffle_address = ctx.accounts.raffle.key().clone();
    let seeds = [
        RAFFLE_POOL_SEED_PREFIX.as_bytes(),
        raffle_address.as_ref(),
        &[raffle.pool_bump],
    ];

    // Transfer token from PDA pool to user ATA
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        TransferSPL {
            from: ctx.accounts.raffle_pool_ata.to_account_info().clone(),
            to: ctx.accounts.dst_ata.to_account_info().clone(),
            authority: ctx.accounts.raffle_pool.to_account_info().clone(),
        }
    );

    // Transfer the token
    token::transfer(cpi_context.with_signer(&[&seeds[..]]), amount)?;

    // If no more balance in token account, let's close it
    if ctx.accounts.raffle_pool_ata.amount == 0
    {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::CloseAccount {
                account: ctx.accounts.raffle_pool_ata.to_account_info().clone(),
                destination: ctx.accounts.admin.to_account_info().clone(),
                authority: ctx.accounts.raffle_pool.to_account_info().clone()
            }
        );
        token::close_account(cpi_context)?;
    }

    Ok(())
}

pub fn raffle_finalize(ctx: Context<RaffleFinalize>, raffle_royalties: u8) -> Result<()> 
{
    /* Transfer raffle winnings minus royalties */
    let raffle = &mut ctx.accounts.raffle;

    require!(raffle.is_finalized == false, ErrorCode::RaffleFinalized);

    raffle.is_finalized = true;

    /* Transfer NFT to winner only if 'winner_nft_ata' is set */
    if ctx.accounts.winner_nft_ata.key() != Pubkey::default()
    {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferSPL {
                    from: ctx.accounts.raffle_nft_ata.to_account_info(),
                    to: ctx.accounts.winner_nft_ata.to_account_info(),
                    authority: ctx.accounts.raffle_bank.to_account_info(),
                },
            ),
            1,
        )?;
    }

    

    // Total raffle bank balance
    let raffle_bank_balance = raffle.price_per_ticket.checked_mul(raffle.sold_tickets as u64).unwrap();

    let mut bank_amount_royalties = 0;
    let mut owner_payout_amount= 0;

    if raffle_bank_balance > 0 // Only send money if there is any earnings in raffle bank
    {
        // Calculate royalties
        bank_amount_royalties = raffle_bank_balance.checked_mul(raffle_royalties as u64).unwrap().checked_div(100).unwrap();

        // Owner payout amount to transfer
        owner_payout_amount = raffle_bank_balance.checked_mul(100).unwrap()
            .checked_sub
            (
                raffle_bank_balance.checked_mul(raffle_royalties as u64).unwrap()
            )
            .unwrap()
            .checked_div(100).unwrap();

        if raffle.token_spl_address == Pubkey::default() // transfer winning via SOL
        {
            // Transfer via SOL
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    TransferProgramSOL {
                        from: ctx.accounts.raffle_bank.to_account_info().clone(),
                        to: ctx.accounts.owner.to_account_info().clone(),
                    },
                ),
                owner_payout_amount,
            )?;
        }
        else // Transfer winning via SPL-Token
        {
            // Transfer via SPL-Token
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    TransferSPL {
                        from: ctx.accounts.raffle_spl_ata.to_account_info(),
                        to: ctx.accounts.owner_spl_ata.to_account_info(),
                        authority: ctx.accounts.raffle_bank.to_account_info(),
                    },
                ),
                owner_payout_amount,
            )?;
        }
    }

    msg!("Version: {:?}", VERSION);
    msg!("Raffle Bank Balance: {:?} ({:?})", utils::to_float(raffle_bank_balance), raffle_bank_balance);
    msg!("Raffle Bank Royalties: {:?}% = {:?} ({:?})", raffle_royalties, utils::to_float(bank_amount_royalties), bank_amount_royalties);
    msg!("Payment Token: {:?}", raffle.token_spl_address);
    msg!("Winner NFT: {:?}", ctx.accounts.winner_nft_ata.owner.key());
    msg!("Winner NFT ATA: {:?}", ctx.accounts.winner_nft_ata.to_account_info().key());
    msg!("Raffle Owner: {:?}", raffle.owner);
    msg!("Raffle Owner Payout: {:?} ({:?})", utils::to_float(owner_payout_amount), owner_payout_amount);

    Ok(())
}