pub const VERSION: &str = "v6";
pub const GAME_SEED_PREFIX: &str = "coinflip_game_pda";
pub const PLAYER_SEED_PREFIX: &str = "player_pda";

pub const APPROVED_WALLETS: [&str; 2] = [
  "SERVUJeqsyaJTuVuXAmmko6kTigJmxzTxUMSThpC2LZ",
  "EF5qxGB1AirUH4ENw1niV1ewiNHzH2fWs7naQQYF2dc"
];

pub const BET_PRICES: [u64; 6] = [
     50_000_000,
    100_000_000,
    250_000_000,
    500_000_000,
  1_000_000_000,
  2_000_000_000,
];