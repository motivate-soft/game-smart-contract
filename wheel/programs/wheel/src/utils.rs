use anchor_lang::{
  prelude::*, 
  solana_program::{
    clock,
    keccak::{hash, Hash}
  }
};
use crate::constants::*;

pub fn get_random() -> u32 {
  let slot = clock::Clock::get().unwrap().unix_timestamp as u64;
  let hash = hash(&slot.to_be_bytes());
  let buf: [u8; 32] = Hash::to_bytes(hash);
  let slice: [u8; 4] = [buf[10], buf[12], buf[8], buf[16]];
  u32::from_be_bytes(slice)  
}
