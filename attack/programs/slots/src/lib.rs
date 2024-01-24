use anchor_lang::prelude::*;

declare_id!("6sE2DYexXa8oBPfGjgoCkNceHgH3xXnXD2nBz7i3NTWE");

#[program]
pub mod slots {
    use super::*;

    pub fn play(ctx: Context<Play>, bet_no: u8) -> Result<()> {
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
    /// CHECK: instruction_sysvar_account cross checking 
    pub instruction_sysvar_account: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
