use anchor_lang::prelude::*;
use crate::ins::*;
use crate::constants::*;
use crate::state::{ErrorCode};

pub fn initialize_global(ctx: Context<InitializeGlobal>) -> Result<()>
{
    let global = &mut ctx.accounts.global;
    global.authority = ctx.accounts.payer.key();
    global.authorized_admins.push(ctx.accounts.admin.key());

    msg!("Version: {:?}", VERSION);
    msg!("Global initialized successfully.");
    msg!("Global Account: {}", ctx.accounts.global.to_account_info().key());

    Ok(())
}

pub fn authorize_admin(ctx: Context<ControlAdmins>) -> Result<()>
{
    let global = &mut ctx.accounts.global;

    if global.authorized_admins.iter().any(|x| x == &ctx.accounts.admin.key())
    {
      return Err(ErrorCode::AlreadyAuthorizedAdmin.into());
    }

    global.authorized_admins.push(ctx.accounts.admin.key());

    Ok(())
}

pub fn unauthorize_admin(ctx: Context<ControlAdmins>) -> Result<()>
{
    let global = &mut ctx.accounts.global;

    let index = global.authorized_admins.iter().position(|x| x == &ctx.accounts.admin.key());
    if let Some(index) = index
    {
      global.authorized_admins.remove(index);
    }
    else
    {
      return Err(ErrorCode::NotAuthorizedAdmin.into());
    }

    Ok(())
}