mod ins;
mod state;
mod utils;
mod constants;

use anchor_lang::{prelude::*};
use anchor_spl::token::{Transfer, CloseAccount, close_account, transfer};

use crate::ins::*;
use crate::constants::*;
use crate::utils::*;
use crate::state::{ErrorCode, Game, Cell};

declare_id!("HsDXU6Ku6ggbqDzhxyrNLjby5MLdT7RXXnR6AhFChD7L");

#[program]
pub mod wheel {
    use super::*;

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn create_game(
        ctx: Context<CreateGame>, 
        name: String, 
        bump: u8,
        spl_mint: Pubkey,
        community_wallets: Vec<Pubkey>, 
        royalties: Vec<u16>,
        commission_wallet: Pubkey,
        commission_fee: u16,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authority = *ctx.accounts.payer.key;
        game.bump = bump;
        game.name = name;
        game.spl_mint = spl_mint;
        game.community_wallets = community_wallets;
        game.commission_wallet = commission_wallet;
        game.commission_fee = commission_fee;
        let len = royalties.len();
        game.royalties = royalties;
        game.main_balance = 0;
        game.community_balances = vec![0; len]; 
        game.community_pending_balances = vec![0; len];

        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.funder))]
    pub fn fund_prize(ctx: Context<FundPrize>, index: usize, amount: u64, chance: u16) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let cell = Cell {
            spl_mint: ctx.accounts.prize_mint.key(),
            amount,
            chance
        };
        game.cells[index] = cell;
    
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info().clone(),
                Transfer {
                    authority: ctx.accounts.funder.to_account_info().clone(),
                    from: ctx.accounts.funder_ata.to_account_info().clone(),
                    to: ctx.accounts.game_ata.to_account_info().clone(),
                }
            ),
            amount,
        )?;

        if ctx.accounts.game_ata.amount == 0 {
            close_account(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info().clone(), 
                    CloseAccount {
                        account: ctx.accounts.funder_ata.to_account_info().clone(),
                        destination: ctx.accounts.funder.to_account_info().clone(),
                        authority: ctx.accounts.funder.to_account_info().clone(),                        
                    }
                )
            )?;
        }
        
        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.withdrawer))]
    pub fn withdraw_prize(ctx: Context<WithdrawPrize>, index: usize) -> Result<()> {
        let game = &ctx.accounts.game;
        let amount = game.cells[index].amount;

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
                ctx.accounts.token_program.to_account_info().clone(),
                Transfer {
                    authority: ctx.accounts.game.to_account_info().clone(),
                    from: ctx.accounts.game_ata.to_account_info().clone(),
                    to: ctx.accounts.withdrawer_ata.to_account_info().clone(),
                }
            ).with_signer(&[&seeds[..]]),
            amount,
        )?;

        if ctx.accounts.game_ata.amount == 0 {
            close_account(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info().clone(), 
                    CloseAccount {
                        account: ctx.accounts.game_ata.to_account_info().clone(),
                        destination: ctx.accounts.withdrawer.to_account_info().clone(),
                        authority: ctx.accounts.game.to_account_info().clone(),                        
                    }
                ).with_signer(&[&seeds[..]])
            )?;
        }
    
        let game = &mut ctx.accounts.game;        
        game.cells[index] = Cell::default();

        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn set_community_wallet(ctx: Context<ConfigGame>, community_wallet: Pubkey, royalty: u16) -> Result<()> {
        let game = &mut ctx.accounts.game;
        
        let index = game.community_wallets.iter().position(|x| x == &community_wallet);
        if let Some(index) = index {
            game.royalties[index] = royalty;
            if royalty == 10001 {
                game.royalties.remove(index);
                game.community_balances.remove(index);
                game.community_wallets.remove(index);
                game.community_pending_balances.remove(index);
                msg!("Removed");
            } else {
                msg!("Updated");
                msg!("New Royalty: {:?}", royalty);
            }
        } else {
            game.community_wallets.push(community_wallet);
            game.royalties.push(royalty);
            game.community_balances.push(0);
            game.community_pending_balances.push(0);
            msg!("New Added");
        }
        msg!("Community Wallet: {:?}", community_wallet);
        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn set_commission(ctx: Context<ConfigGame>, commission_wallet: Pubkey, commission_fee: u16) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.commission_wallet = commission_wallet;
        game.commission_fee = commission_fee;

        Ok(())
    }
    
    pub fn add_player(ctx: Context<AddPlayer>, bump: u8) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.key = ctx.accounts.payer.key();
        player.game = ctx.accounts.game.key();
        player.bump = bump;
        player.status = 0;

        Ok(())
    }

    pub fn send_to_community_wallet(ctx: Context<SendToCommunityWallet>) -> Result<()> {
        let game = &ctx.accounts.game;
        let index = game.community_wallets.iter().position(|x| x == &ctx.accounts.community_wallet.key());
        if let Some(index) = index {
            let amount = game.community_pending_balances[index];
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
                        to: ctx.accounts.community_treasury_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                amount,
            )?;
            let game = &mut ctx.accounts.game;
            game.community_pending_balances[index] = 0;
            game.community_balances[index] = game.community_balances[index].checked_add(amount).unwrap();
        }
        Ok(())
    }

    pub fn play(ctx: Context<Play>) -> Result<()> {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        let price: u64 = 50_000_000;

        player.status = get_random();
        msg!("Player PDA: {:?}", player.key);

        let commission_amount = price.checked_mul(game.commission_fee as u64).unwrap().checked_div(10000).unwrap();        
        
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.payer.to_account_info().clone(),
                    from: ctx.accounts.payer_ata.to_account_info().clone(),
                    to: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                }
            ),
            price.checked_sub(commission_amount).unwrap(),
        )?;
        if commission_amount > 0 {
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.payer.to_account_info().clone(),
                        from: ctx.accounts.payer_ata.to_account_info().clone(),
                        to: ctx.accounts.commission_treasury_ata.to_account_info().clone(),
                    }
                ),
                price.checked_sub(commission_amount).unwrap(),
            )?;
        }

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