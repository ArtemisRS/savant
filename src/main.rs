#![allow(unused)]

use anyhow::Result;
use randomize::{formulas::f32_half_open_left, RandRangeU32, PCG32};
use serde::Deserialize;
use std::fs;
use simulator::*;
use zulrah::{Return, Rot, Zulrah};

mod zulrah;
mod simulator;

#[derive(Debug, Deserialize)]
struct Config {

    name: String,
    //future options
    //rotation: Option<zulrah::Rot>, //specify a specific rotation
    //beat_time: Option<u8>, //in ticks, for $ of kills faster than time
    //stall: Option<bool>,
    //attacks: Option<Vec<setup-ids>>, //list out every attack

    serpentine: Setup,
    crimson: Setup,
    tanzanite: Setup,
}

#[derive(Debug, Deserialize)]
pub struct Setup {
    accuracy: f32,
    max_hit: u16,
    delay: u16,
}


fn main() -> Result<()> {
    let input = fs::read_to_string("config").unwrap_or_else(|_|"Missing config file?".to_string());

    let config: Config = toml::from_str(&input)?;

    dbg!(&config);
    simulator::main(&config.serpentine, &config.crimson, &config.tanzanite);
    Ok(())
}
