mod ins;
mod state;
mod constants;

use anchor_lang::{
    prelude::*,    
};
use anchor_spl::token::{transfer, Transfer};

use ins::*;
use constants::*;
use state::ErrorCode;

declare_id!("3oKv371BCsfL7br4xtUE3RaQmdHzJTv5FJnsK5wQNRBd");

#[program]
pub mod plinko {
    use super::*;

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn create_game(
        ctx: Context<CreateGame>, 
        name: String, 
        bump: u8, 
        backend_wallet: Pubkey,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.payer.key();
        game.name = name;
        game.bump = bump;
        game.backend_wallet = backend_wallet;
        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn set_backend_wallet(ctx: Context<ConfigGame>, backend_wallet: Pubkey) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.backend_wallet = backend_wallet;
        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.claimer))]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        msg!("Version: {:?}", VERSION);

        let game = &ctx.accounts.game;
        let game_name = &game.name;
        let authority = game.authority;
        let seeds = [
            game_name.as_bytes(),
            GAME_SEED_PREFIX.as_bytes(),
            authority.as_ref(),
            &[game.bump]
        ];

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.game.to_account_info().clone(),
                    from: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                    to: ctx.accounts.claimer_ata.to_account_info().clone(),
                }
            ).with_signer(&[&seeds[..]]),
            amount,
        )?; 

        Ok(())
    }

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.payer.to_account_info().clone(),
                    from: ctx.accounts.payer_ata.to_account_info().clone(),
                    to: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                }
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, amount: u64) -> Result<()> {
        let game = &ctx.accounts.game;
        
        msg!("Version: {:?}", VERSION);

        let game_name = &game.name;
        let authority = game.authority;
        let seeds = [
            game_name.as_bytes(),
            GAME_SEED_PREFIX.as_bytes(),
            authority.as_ref(),
            &[game.bump]
        ];

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.game.to_account_info().clone(),
                    from: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                    to: ctx.accounts.claimer_ata.to_account_info().clone(),
                }
            ).with_signer(&[&seeds[..]]),
            amount,
        )?;

        Ok(())
    }
}

pub fn authorized_admin(admin: &AccountInfo) -> Result<()> {
    let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == admin.key());
    if index == false {
        return Err(ErrorCode::UnauthorizedWallet.into());
    }
    Ok(())
}