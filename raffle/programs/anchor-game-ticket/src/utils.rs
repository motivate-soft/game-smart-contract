use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_lang::solana_program::program::invoke;
use spl_memo::build_memo;
use crate::constants::LAMPORTS_PER_SOL;

use crate::ins::*;

pub fn transfer_spl_token(ctx: Context<TransferSPLToken>, amount: u64) -> Result<()>
{
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.recipient_tokens.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!("Transfer {} Done!",  ctx.accounts.recipient_tokens.mint);
    msg!("System ID {}!",  &System::id());

    Ok(())
}

pub fn memo(ctx: Context<Memo>, text: &str) -> Result<()> {
    let account_info = vec![
        ctx.accounts.memo.to_account_info()
    ];

    invoke(
        &build_memo(text.as_bytes(), &[]),
        account_info.as_slice()
    )?;

    Ok(())
}

pub fn to_float(num: u64) -> f64
{
    return num as f64 / LAMPORTS_PER_SOL as f64;
}

