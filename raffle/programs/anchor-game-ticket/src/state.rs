use anchor_lang::prelude::*;

#[account]
pub struct Vault
{
    pub token_type: Pubkey,
    pub vault_bump: u8,
}

impl Vault
{
    pub const LEN: usize = std::mem::size_of::<Vault>();
}

#[account]
pub struct Global {
    pub authority: Pubkey,
    pub authorized_admins: Vec<Pubkey>,
}

impl Global {
    pub const LEN: usize = 1 + 32 * 5; // allow 5 auth admins
}

#[account]
pub struct Raffle
{
    pub pool_bump: u8,
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub price_per_ticket: u64,
    pub token_spl_address: Pubkey,
    pub owner: Pubkey,
    pub nft_mint_address: Pubkey,
    pub store_buyers: bool,
    pub is_finalized: bool,
    pub buyers: Vec<Buyer>,
}

impl Raffle
{
    pub const SPACE: usize =  std::mem::size_of::<Raffle>();
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy)]
pub struct Buyer {
    pub key: Pubkey,
    pub tickets: u32,
}

#[event]
pub struct BuyEvent
{
    pub buyer: Pubkey,
    pub amount: u32,
    pub sold_tickets: u32,
    pub total_tickets: u32,
    pub remaining_tickets: u32
}

#[event]
pub struct ClaimSKTEvent
{
    pub claimer: Pubkey,
    pub amount: u64
}

#[error_code]
pub enum ErrorCode
{
    #[msg("No more tickets left for purchase.")] // 0x1770 - 6000
    NoTicketsLeft,
    #[msg("Raffle price mismatched.")] // 0x1771 - 6001
    RafflePriceMismatched,
    #[msg("Token Address mismatched.")] // 0x1772 - 6002
    RaffleTokenSPLAddressMismatched,
    #[msg("Not Enough Tokens.")] // 0x1773 - 6003
    NotEnoughTokens,
    #[msg("Custom Error.")] // 0x1774 - 6004
    ErrorCustom,
    #[msg("Already authorized admin")] // 0x1775 - 6005
    AlreadyAuthorizedAdmin,
    #[msg("Not authorized admin")] // 0x1776 - 6006
    NotAuthorizedAdmin,
    #[msg("Cannot withdraw more than 10,000")] // 0x1777 - 6007
    ExceedMaxWithdrawAmount,
    #[msg("Raffle already finalized.")] // 0x1778 - 6008
    RaffleFinalized
}