use anchor_lang::prelude::*;
use slots::{self, program::Slots};

declare_id!("F1Bpf6SjcUzwXj6EPKUg4gNJazZytN5pjvVrFJw6ofxm");

#[program]
pub mod attack {
    use super::*;

    pub fn play(ctx: Context<Play>, bet_no: u8) -> Result<()> {
        let cpi_program = ctx.accounts.slots_program.to_account_info();
        let cpi_accounts = slots::cpi::accounts::Play {
            payer: ctx.accounts.payer.to_account_info(),
            payer_ata: ctx.accounts.payer_ata.to_account_info(),
            game: ctx.accounts.game.to_account_info(),
            game_treasury_ata: ctx.accounts.game_treasury_ata.to_account_info(),
            commission_treasury: ctx.accounts.commission_treasury.to_account_info(),
            commission_treasury_ata: ctx.accounts.commission_treasury_ata.to_account_info(),
            player: ctx.accounts.player.to_account_info(),
            instruction_sysvar_account: ctx.accounts.instruction_sysvar_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        slots::cpi::play(cpi_ctx, bet_no);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Play<'info> 
{
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub payer_ata: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub game: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub game_treasury_ata: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub commission_treasury: SystemAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub commission_treasury_ata: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub player: AccountInfo<'info>,
    pub slots_program: Program<'info, Slots>,
    /// CHECK: instruction_sysvar_account cross checking 
    pub instruction_sysvar_account: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
