#![allow(unused)]

use randomize::{formulas::f32_half_open_left, RandRangeU32, PCG32};
use zulrah::{Return, Rot, Zulrah};
use simulator::*;

mod zulrah;
mod simulator;


fn main() {
    simulator::main();
}
