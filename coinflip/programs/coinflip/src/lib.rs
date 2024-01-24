mod ins;
mod state;
mod constants;
mod utils;

use anchor_lang::{
    prelude::*,
    solana_program::{
        serialize_utils::read_u16,
        sysvar::instructions::{
            get_instruction_relative,
            load_current_index_checked,
            load_instruction_at_checked,
        }
    }
};
use anchor_spl::token::{transfer, Transfer};

use ins::*;
use utils::*;
use constants::*;
use state::ErrorCode;

declare_id!("75Jrbo4F3tTbM7Kd84HC3NXpFCYjGCJohsjhKvNzMh2X");

#[program]
pub mod coinflip {
    use super::*;

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn create_game(
        ctx: Context<CreateGame>, 
        name: String, 
        bump: u8, 
        token_mint: Pubkey,
        community_wallets: Vec<Pubkey>, 
        royalties: Vec<u16>,
        commission_wallet: Pubkey,
        commission_fee: u16,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.payer.key();
        game.name = name;
        game.bump = bump;
        game.token_mint = token_mint;
        game.community_wallets = community_wallets;
        game.commission_wallet = commission_wallet;
        game.commission_fee = commission_fee;
        let len = royalties.len();
        game.community_balances = vec![0; len]; 
        game.community_pending_balances = vec![0; len];
        game.royalties = royalties;
        game.main_balance = 0;
        game.win_percents = [
            4750,
            4750,
            4000,
            4000,
            2500,
            1667,
        ];
        game.total_round = 0;
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

    #[access_control(authorized_admin(&ctx.accounts.payer))]
    pub fn set_winning(ctx: Context<ConfigGame>, win_percents: [u16; 6]) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.win_percents = win_percents;
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>, bump: u8) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.key = ctx.accounts.payer.key();
        player.game = ctx.accounts.game.key();
        player.bump = bump;
        player.earned_money = 0;
        player.rand = 0;
        player.bet_amount = 0;
        player.bet_number = 0;
        player.round = 0;
        player.played_time = 0;
        player.reward_amount = 0;

        Ok(())
    }

    #[access_control(valid_program(&ctx.accounts.instruction_sysvar_account, *ctx.program_id))]
    pub fn play(ctx: Context<Play>, bet_amount: u8, bet_number: u8) -> Result<()> {
        if bet_amount > 5 {
            return Err(ErrorCode::MinimumPrice.into());
        }
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        let win_percents = game.win_percents;
        let price = BET_PRICES[bet_amount as usize];
        let (rand, earned) = get_status(bet_amount, &ctx.accounts.recent_slothashes, win_percents);
        player.rand = rand;
        player.bet_amount = bet_amount;
        player.bet_number = bet_number;
        player.played_time = now();
        player.reward_amount = earned;

        msg!("Version: {:?}", VERSION);
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
                commission_amount,
            )?;
        }

        let game = &mut ctx.accounts.game;
        game.total_round = game.total_round.checked_add(1).unwrap();
        game.main_balance = game.main_balance.checked_add(price).unwrap().checked_sub(commission_amount).unwrap();
        let len = game.royalties.len();
        for i in 0..len {
            let royalty = game.royalties[i];
            let royalty_amount = price.checked_mul(royalty as u64).unwrap().checked_div(10000).unwrap();
            game.community_pending_balances[i] = game.community_pending_balances[i].checked_add(royalty_amount).unwrap();
            game.main_balance = game.main_balance.checked_sub(royalty_amount).unwrap();
        }
        player.earned_money = player.earned_money.checked_add(earned).unwrap();
        
        Ok(())
    }

    pub fn send_to_community_wallet(ctx: Context<SendToCommunityWallet>) -> Result<()> {
        let game = &ctx.accounts.game;
        let index = game.community_wallets.iter().position(|x| x == &ctx.accounts.community_treasury_ata.owner);
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
    
    #[access_control(valid_program(&ctx.accounts.instruction_sysvar_account, *ctx.program_id))]
    #[access_control(prevent_prefix_instruction(&ctx.accounts.instruction_sysvar_account))]
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        
        let amount = player.earned_money;
        player.earned_money = 0;

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

        let game = &mut ctx.accounts.game;
        game.main_balance = game.main_balance.checked_sub(amount).unwrap();
        
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
     
        let game = &mut ctx.accounts.game;
        game.main_balance = game.main_balance.checked_sub(amount).unwrap();

        Ok(())
    }

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.main_balance = game.main_balance.checked_add(amount).unwrap();

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
}

pub fn authorized_admin(admin: &AccountInfo) -> Result<()> {
    let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == admin.key());
    if index == false {
        return Err(ErrorCode::UnauthorizedWallet.into());
    }
    Ok(())
}

pub fn prevent_prefix_instruction(instruction_sysvar_account: &AccountInfo) -> Result<()> {
    let invalid = match get_instruction_relative(-1, &instruction_sysvar_account) {
        Ok(_) => true,
        Err(_) => false,
    };
    if invalid == true {
        return Err(ErrorCode::InvalidInstructionAdded.into());
    }
    // let invalid = match get_instruction_relative(1, &instruction_sysvar_account) {
    //     Ok(_) => true,
    //     Err(_) => false,
    // };
    // if invalid == true {
    //     return Err(ErrorCode::InvalidInstructionAdded.into());
    // }
    Ok(())
}

pub fn valid_program(instruction_sysvar_account: &AccountInfo, program_id: Pubkey) -> Result<()> {
    let data = instruction_sysvar_account.try_borrow_data()?;
    let current_index = load_current_index_checked(instruction_sysvar_account.to_account_info().as_ref())?;
    msg!("Current index {}", current_index);
    let mut current = 0;
    let num_instructions;
    match read_u16(&mut current, &**data) {
        Ok(index) => {
            num_instructions = index;
        }
        Err(_) => {
            return Err(ErrorCode::InvalidProgramId.into());
        }
    }
    msg!("Num instructions {}", num_instructions);
    let valid_program_ids = &[
        program_id,
        Pubkey::default(),
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".parse::<Pubkey>().unwrap(),
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".parse::<Pubkey>().unwrap(),
    ];
    for index in 0..num_instructions {
        msg!("index {}", index);
        let instruction = load_instruction_at_checked(index as usize, &instruction_sysvar_account)?;
        msg!("Program ID {}", instruction.program_id);
        if valid_program_ids.iter().any(|x| x == &instruction.program_id) == false {
            return Err(ErrorCode::InvalidProgramId.into());
        }
    }
    Ok(())
}