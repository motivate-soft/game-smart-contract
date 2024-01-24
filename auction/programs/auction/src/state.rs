use anchor_lang::prelude::*;

#[account]
pub struct Auction
{
    pub name: String,

    pub creator: Pubkey,

    pub min_bid_price: u64,

    pub auction_started_time: u64,

    pub auction_finish_time: u64,

    pub nft_mint: Pubkey,

    pub spl_token_mint: Pubkey,

    pub last_bidder: Pubkey,

    pub transfered_to_winner: bool,

    pub bump: u8,
}

impl Auction {
    pub const LEN: usize = std::mem::size_of::<Auction>();
}

#[error_code]
pub enum CustomError {
    #[msg("Smaller than min bid price")]
    MinBidPrice,
    #[msg("Auction is finished already")]
    AuctionFinished,
    #[msg("Auction is not finished yet")]
    AuctionNotFinished,
    #[msg("Transfer to winner first")]
    TransferedToWinner,
    #[msg("Unathorized Wallet")]
    UnauthorizedWallet,
}