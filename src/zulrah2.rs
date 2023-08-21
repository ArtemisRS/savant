use std::collections::VecDeque;
use std::fmt;
use smallvec::SmallVec;
use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub enum Form {
    Serpentine, //green
    Magma,      //red
    Tanzanite,  //blue
}

//Zulrah spawn point for a phase
#[derive(Debug, Clone)]
enum Location {
    Middle,
    East,
    West,
    South,
}

//one spawn of a particular rotation - lasts from dive to dive
#[derive(Debug, Clone)]
struct Phase {
    form: Form,
    loc: Location,
    stallable_attacks: u16 //take 3t, can be stalled to 7t
    basic_attacks: u16 // take 4t?
    time: u16, //ticks
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = write!(f, "{:?} {:?}", self.loc, self.form);
        if !self.damage.is_empty() {
            write!(
                f,
                "\n  {} damage - {:?}",
                self.damage.iter().sum::<u16>(),
                self.damage,
            )?;
        }
        res
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Rot {
    Southgreen,
    Westgreen,
    Eastgreen,
    Eastblue,
}

impl fmt::Display for Rot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Rot::Southgreen => write!(f, "Rotation 1: South Green"),
            Rot::Westgreen => write!(f, "Rotation 2: West Green"),
            Rot::Eastgreen => write!(f, "Rotation 3: East Green"),
            Rot::Eastblue => write!(f, "Rotation 4: East Blue"),
        }
    }
}
