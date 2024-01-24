use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::{Create, create};
use anchor_spl::token;
use crate::state::{ClaimSKTEvent, ErrorCode};
use crate::constants::*;
use crate::utils::*;
use crate::ins::*;
use crate::id;

pub fn initialize_vault(ctx: Context<InitializeVault>, token_type: Pubkey, vault_bump: u8) -> Result<()>
{
    if ctx.accounts.vault_pool.owner == &System::id() {
        let cpi_context = CpiContext::new(
            ctx.accounts.associated_token.to_account_info(),
            Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.vault_pool_skt_account.to_account_info(),
                authority: ctx.accounts.vault_pool.to_account_info(),
                mint: ctx.accounts.skt_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );
        create(cpi_context)?;
    }

    let vault = &mut ctx.accounts.vault;
    vault.token_type = token_type;
    vault.vault_bump = vault_bump;

    msg!("Version: {:?}", VERSION);
    msg!("Vault Address: {:?}", vault.key().clone());
    msg!("Vault PDA: {:?}", ctx.accounts.vault_pool.key);
    msg!("Vault ATA: {:?}", ctx.accounts.vault_pool_skt_account.key);
    msg!("Vault Owner: {:?}", ctx.accounts.vault_pool.owner);
    msg!("System ID: {:?}", &System::id());

    memo(
        Context::new(
            &id(),
            &mut Memo {
                memo: ctx.accounts.memo.clone()
            },
            &[],
            ctx.bumps.clone()
        ),
        "Vault Created"
    )?;

    Ok(())
}

pub fn withdraw_vault(ctx: Context<WithdrawVault>, spl_amount: u64, sol_amount: u64) -> Result<()>
{
    let global = &ctx.accounts.global;
    if global.authorized_admins.iter().any(|x| x == &ctx.accounts.claimer.key()) == false {
        return Err(ErrorCode::NotAuthorizedAdmin.into());
    }

    if spl_amount >= 10_000 * LAMPORTS_PER_SOL {
        return Err(ErrorCode::ExceedMaxWithdrawAmount.into())
    }

    if spl_amount > 0 {
        let vault = &ctx.accounts.vault;
        let vault_address = vault.key().clone();
    
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            token::Transfer
            {
                from: ctx.accounts.vault_pool_skt_account.to_account_info().clone(),
                to: ctx.accounts.claimer_skt_account.to_account_info().clone(),
                authority: ctx.accounts.vault_pool.to_account_info().clone(),
            }
        );
    
        let seeds = [
            VAULT_SKT_SEED_PREFIX.as_bytes(),
            vault_address.as_ref(),
            &[vault.vault_bump],
        ];
        token::transfer(cpi_context.with_signer(&[&seeds[..]]), spl_amount)?;
    }

    if sol_amount > 0 {
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= sol_amount;
        **ctx.accounts.claimer.to_account_info().try_borrow_mut_lamports()? += sol_amount;
    }

    Ok(())
}

pub fn claim_skt(ctx: Context<ClaimSkt>, amount: u64) -> Result<()>
{
    let vault = &ctx.accounts.vault;
    let vault_address = vault.key().clone();

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        token::Transfer
        {
            from: ctx.accounts.vault_pool_skt_account.to_account_info().clone(),
            to: ctx.accounts.claimer_skt_account.to_account_info().clone(),
            authority: ctx.accounts.vault_pool.to_account_info().clone(),
        }
    );

    let seeds = [
        VAULT_SKT_SEED_PREFIX.as_bytes(),
        vault_address.as_ref(),
        &[vault.vault_bump],
    ];
    token::transfer(cpi_context.with_signer(&[&seeds[..]]), amount)?;

    emit!(ClaimSKTEvent
         {
            claimer: ctx.accounts.claimer.key.clone(),
            amount: amount
         });

    Ok(())
}

pub fn convert_skt_sol(ctx: Context<Convert>, exchange_option: u8, is_holder: bool) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let vault_address = vault.key().clone();
    
    let sol_amount = match is_holder
    {
        false => match exchange_option
        {
            0 => 500_000_000,
            1 => 700_000_000,
            2 => 1_200_000_000,
            _ => 1_800_000_000
        },
        true => match exchange_option
        {
            0 => 400_000_000,
            1 => 600_000_000,
            2 => 1_000_000_000,
            _ => 1_600_000_000
        }
    };

    let skt_amount = match exchange_option
    {
        0 => 70_000_000_000,
        1 => 140_000_000_000,
        2 => 320_000_000_000,
        _ => 500_000_000_000
    };

    // Send SOL from buyer to vault
    {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info().clone(),
            system_program::Transfer {
                from: ctx.accounts.claimer.to_account_info().clone(),
                to: ctx.accounts.vault.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_context, sol_amount)?;
    }

    // Send SKT to buyer
    {
        if ctx.accounts.claimer_skt_account.owner == &System::id() {
            let cpi_context = CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info().clone(),
                Create {
                    payer: ctx.accounts.claimer.to_account_info().clone(),
                    associated_token: ctx.accounts.claimer_skt_account.to_account_info().clone(),
                    authority: ctx.accounts.claimer.to_account_info().clone(),
                    mint: ctx.accounts.skt_mint.to_account_info().clone(),
                    rent: ctx.accounts.rent.to_account_info().clone(),
                    token_program: ctx.accounts.token_program.to_account_info().clone(),
                    system_program: ctx.accounts.system_program.to_account_info().clone(),
                }
            );
            create(cpi_context)?;
        }

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_pool_skt_account.to_account_info().clone(),
                to: ctx.accounts.claimer_skt_account.to_account_info().clone(),
                authority: ctx.accounts.vault_pool.to_account_info().clone(),
            }
        );

        let seeds = [
            VAULT_SKT_SEED_PREFIX.as_bytes(),
            vault_address.as_ref(),
            &[vault.vault_bump],
        ];

        token::transfer(cpi_context.with_signer(&[&seeds[..]]), skt_amount)?;

        msg!("Version: {:?}", VERSION);
        msg!("Vault Address: {:?}", vault_address);
        msg!("Vault PDA: {:?}", ctx.accounts.vault_pool.key);
        msg!("Vault ATA: {:?}", ctx.accounts.vault_pool_skt_account.key());
        msg!("Vault Owner: {:?}", ctx.accounts.vault_pool.owner);
        msg!("Received: {:?} ({:?} SOL)", sol_amount, sol_amount as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Sent: {:?} ({:?} $SKT)", skt_amount, skt_amount / LAMPORTS_PER_SOL);
    }

    Ok(())
}