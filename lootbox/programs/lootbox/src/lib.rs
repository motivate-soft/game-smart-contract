mod ins;
mod state;
mod constants;
mod utils;

use crate::ins::*;
use crate::state::*;
use crate::constants::*;
use crate::utils::*;

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


declare_id!("A6woh5LgWEmPnQmR4rz3XyxAVs5VdNbHKqgKN3aaNjR3");

#[program]
pub mod lootbox {
    use super::*;

    pub fn create_lootbox(
        ctx: Context<CreateLootbox>, 
        name: String, 
        image_url: String,
        token_mint: Pubkey,
        price: u64,        
        win_percent: u16,
    ) -> Result<()> {
        let lootbox = &mut ctx.accounts.lootbox;
        lootbox.authority = ctx.accounts.authority.key();
        lootbox.name = name;
        lootbox.image_url = image_url;        
        lootbox.win_percent = win_percent;
        lootbox.price = price;

        lootbox.balance = Balance {
            token_mint: token_mint,
            amount: 0,
        };
        
        lootbox.bump = *ctx.bumps.get(GAME_SEED_PREFIX).unwrap();
        Ok(())
    }

    pub fn set_winning(ctx: Context<ConfigLootbox>, image_url: String, win_percent: u16, price: u64) -> Result<()> {
        let lootbox = &mut ctx.accounts.lootbox;
        lootbox.win_percent = win_percent;
        lootbox.price = price;
        lootbox.image_url = image_url;

        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.key = ctx.accounts.payer.key();
        player.balances = vec![];

        player.bump = *ctx.bumps.get(PLAYER_SEED_PREFIX).unwrap();

        Ok(())
    }

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        let lootbox = &mut ctx.accounts.lootbox;
        lootbox.balance.amount = lootbox.balance.amount.checked_add(amount).unwrap();

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.payer.to_account_info().clone(),
                    from: ctx.accounts.payer_ata.to_account_info().clone(),
                    to: ctx.accounts.lootbox_ata.to_account_info().clone(),
                }
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let lootbox = &ctx.accounts.lootbox;
        let lootbox_name = &lootbox.name;
        let authority = lootbox.authority;
        let seeds = [
            GAME_SEED_PREFIX.as_bytes(),
            lootbox_name.as_bytes(),
            authority.as_ref(),
            &[lootbox.bump]
        ];

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.lootbox.to_account_info().clone(),
                    from: ctx.accounts.lootbox_ata.to_account_info().clone(),
                    to: ctx.accounts.claimer_ata.to_account_info().clone(),
                }
            ).with_signer(&[&seeds[..]]),
            amount,
        )?; 
     
        let lootbox = &mut ctx.accounts.lootbox;
        lootbox.balance.amount = lootbox.balance.amount.checked_sub(amount).unwrap();

        Ok(())
    }

    #[access_control(valid_program(&ctx.accounts.instruction_sysvar_account, *ctx.program_id))]
    pub fn play(ctx: Context<Play>) -> Result<()> {
        let lootbox = &ctx.accounts.lootbox;
        let player = &mut ctx.accounts.player;
        let win_percent = lootbox.win_percent;
        let price = lootbox.price;

        if player.balances.iter().any(|x| x.token_mint == lootbox.balance.token_mint) == false {
            player.balances.push(Balance {
                token_mint: lootbox.balance.token_mint,
                amount: 0
            });
        }
        
        let earned = get_status(price, &ctx.accounts.recent_slothashes, win_percent);

        msg!("Version: {:?}", VERSION);
        msg!("Player PDA: {:?}", player.key);

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    authority: ctx.accounts.payer.to_account_info().clone(),
                    from: ctx.accounts.payer_ata.to_account_info().clone(),
                    to: ctx.accounts.lootbox_ata.to_account_info().clone(),
                }
            ),
            price,
        )?;

        if earned > 0 {
            let lootbox_name = &lootbox.name;
            let authority = lootbox.authority;
            let seeds = [
                GAME_SEED_PREFIX.as_bytes(),
                lootbox_name.as_bytes(),
                authority.as_ref(),
                &[lootbox.bump]
            ];

            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.lootbox.to_account_info().clone(),
                        from: ctx.accounts.lootbox_ata.to_account_info().clone(),
                        to: ctx.accounts.player_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                earned,
            )?; 

            let index = player.balances.iter().position(|x| x.token_mint == lootbox.balance.token_mint).unwrap();
            player.balances[index].amount = player.balances[index].amount.checked_add(earned).unwrap();

            let lootbox = &mut ctx.accounts.lootbox;
            lootbox.balance.amount = lootbox.balance.amount.checked_sub(earned).unwrap();
        }
        
        let lootbox = &mut ctx.accounts.lootbox;
        lootbox.balance.amount = lootbox.balance.amount.checked_add(price).unwrap();
        
        Ok(())
    }

    #[access_control(valid_program(&ctx.accounts.instruction_sysvar_account, *ctx.program_id))]
    #[access_control(prevent_prefix_instruction(&ctx.accounts.instruction_sysvar_account))]
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let player = &ctx.accounts.player;
    
        msg!("Version: {:?}", VERSION);

        let index = player.balances.iter().position(|x| x.token_mint == ctx.accounts.token_mint.key()).unwrap();
        let amount = player.balances[index].amount;

        if amount > 0 {
            let player_key = player.key;
            let seeds = [
                PLAYER_SEED_PREFIX.as_bytes(),
                player_key.as_ref(),
                &[player.bump]
            ];
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.player.to_account_info().clone(),
                        from: ctx.accounts.player_ata.to_account_info().clone(),
                        to: ctx.accounts.claimer_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                amount,
            )?;
        }

        let player = &mut ctx.accounts.player;
        player.balances[index].amount = 0;
        
        Ok(())
    }
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
            return Err(CustomError::InvalidProgramId.into());
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
        require!(valid_program_ids.iter().any(|x| x == &instruction.program_id) == true, CustomError::InvalidProgramId);
    }
    Ok(())
}

pub fn prevent_prefix_instruction(instruction_sysvar_account: &AccountInfo) -> Result<()> {
    let invalid = match get_instruction_relative(-1, &instruction_sysvar_account) {
        Ok(_) => true,
        Err(_) => false,
    };
    require!(invalid == false, CustomError::InvalidInstructionAdded);

    Ok(())
}
