use anchor_lang::{
  prelude::*, 
  solana_program::{
    clock,
    keccak::{hash, Hash}
  }
};
use crate::constants::*;

pub fn get_random(index: u64) -> u32 {
  let mut slot = clock::Clock::get().unwrap().unix_timestamp as u64;
  slot = slot.checked_add(index).unwrap();
  let hash = hash(&slot.to_be_bytes());
  let buf: [u8; 32] = Hash::to_bytes(hash);
  let slice: [u8; 4] = [buf[10], buf[12], buf[8], buf[16]];
  u32::from_be_bytes(slice)  
}

pub fn get_status(index: u64, bet_no: u8, bet_prices: &Vec<u64>, win_percents: &Vec<Vec<u16>>, jackpot: u64, lose: bool) -> (u32, u64, bool, u32, u32, u32) {
  let mut rand = get_random(index);
  let bn = bet_no as usize;
  let price = bet_prices[bn];
  
  let mut equal_count = rand % 2 + 1;
  rand = rand % 10000;
  for i in 0..3 {
    let mut low: u32 = 0;
    if i < 2 {
      low = win_percents[bn][i + 1].into();
    }
    if rand >= low && rand < win_percents[bn][i].into() && lose == false {
      equal_count = 3 + i as u32;
    }
  }
  let equal_no = rand % 10;
  let mut multipler = 0;
  let mut earned = 0;
  let mut is_jackpot = false;
  if equal_count >= 3 {
    multipler = (equal_count - 1) * 10 - rand % 10;
    earned = price.checked_mul(multipler as u64).unwrap().checked_div(10).unwrap();
    if equal_count == 5 && jackpot > 0 && bet_no > 3 {
      is_jackpot = true;
      earned = jackpot;
    }
  }

  // msg!("Status: {:?}", rand);
  // msg!("Bet Price: {:?}", price);
  // msg!("Equal Count: {:?}", equal_count);
  // msg!("Equal No: {:?}", equal_no);
  // msg!("Is Jackpot: {:?}", is_jackpot);
  // msg!("Multiplier: {:?}", multipler);
  // msg!("Earned: {:?}", earned);

  return (rand, earned, is_jackpot, equal_count, equal_no, multipler);
}