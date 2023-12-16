#![forbid(unsafe_code)]
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RollDiceConfig {
    pub probas: [f64; 6],
    pub players: (u8, u8),
}
