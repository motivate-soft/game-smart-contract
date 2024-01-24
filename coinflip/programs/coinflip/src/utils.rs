use anchor_lang::{
  prelude::*, 
  solana_program::{
    clock,
  }
};
use arrayref::array_ref;
use crate::constants::*;

pub fn now() -> u64 {
  clock::Clock::get().unwrap().unix_timestamp as u64
}

pub fn get_random(recent_slothashes: &AccountInfo) -> u32 {
  let data = recent_slothashes.data.borrow();
  let most_recent = array_ref![data, 12, 8];
  let timestamp = now();
  let seed = u64::from_le_bytes(*most_recent).saturating_sub(timestamp);
  let remainder: u32 = seed
    .checked_rem(10000).unwrap() as u32;

  remainder
}

pub fn get_status(bet_amount: u8, recent_slothashes: &AccountInfo, win_percents: [u16; 6]) -> (u32, u64) {
  let rand = get_random(recent_slothashes);
  let bn = bet_amount as usize;
  let price = BET_PRICES[bn];

  // rand = rand % 10000;
  let mut earned: u64 = 0;
  if rand < win_percents[bn].into() {
    earned = price.checked_mul(2).unwrap();
  }
  
  msg!("Random: {:?}", rand);
  msg!("Bet Price: {:?}", BET_PRICES[bn]);
  msg!("Reward: {:?}", earned);

  return (rand, earned);
}