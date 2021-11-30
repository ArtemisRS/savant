use std::collections::VecDeque;
use std::fmt;
use smallvec::SmallVec;

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
    time: u16, //ticks
    damage: SmallVec::<[u16; 16]>,
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
        return res;
    }
}

#[derive(Debug, Clone, Copy)]
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

//all of the phases for one Zulrah kill
//start with one rotation and append additional as needed
#[derive(Debug)]
struct Rotation {
    name: Rot,
    phases: VecDeque<Phase>,
    delay: u16,
}

impl Rotation {
    //TODO: should randomly pick one of the four rotations
    fn new(rot: Rot) -> Rotation {
        let mut phases = VecDeque::new();

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 20,
            damage: SmallVec::new(),
        });

        Rotation::add_phases(&rot, &mut phases);
        Rotation {
            name: rot,
            phases,
            delay: 7,
        }
    }

    fn add_phases(rot: &Rot, phases: &mut VecDeque<Phase>) {
        match *rot {
            Rot::Southgreen => Rotation::south_green(phases),
            Rot::Westgreen => Rotation::west_green(phases),
            Rot::Eastgreen => Rotation::east_green(phases),
            Rot::Eastblue => Rotation::east_blue(phases),
        }
    }

    //TODO: should randomly pick the follow on phase
    fn next_phase(&mut self) -> Phase {
        let phase = self.phases.pop_front();
        match phase {
            Some(phase) => phase,
            None => {
                Rotation::add_phases(&self.name, &mut self.phases);
                self.phases.pop_front().unwrap()
            }
        }
    }

    fn south_green(phases: &mut VecDeque<Phase>) {
        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 11,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 33,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 16,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::West,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 22,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::South,
            time: 30,
            damage: SmallVec::new(),
        });

        //jad
        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::West,
            time: 42,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 15,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 28,
            damage: SmallVec::new(),
        });
    }

    fn west_green(phases: &mut VecDeque<Phase>) {
        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 11,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::West,
            time: 22,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::South,
            time: 33,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 15,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::East,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::South,
            time: 30,
            damage: SmallVec::new(),
        });

        //jad
        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::West,
            time: 42,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 15,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 28,
            damage: SmallVec::new(),
        });
    }

    fn east_green(phases: &mut VecDeque<Phase>) {
        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::East,
            time: 24,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 34,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::West,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::East,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 19,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::West,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 28,
            damage: SmallVec::new(),
        });

        //jad
        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::East,
            time: 29,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 12,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 28,
            damage: SmallVec::new(),
        });
    }

    fn east_blue(phases: &mut VecDeque<Phase>) {
        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::East,
            time: 30,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 18,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::West,
            time: 24,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 22,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::East,
            time: 11,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 28,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::West,
            time: 27,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 14,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 21,
            damage: SmallVec::new(),
        });

        //jad
        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::East,
            time: 23,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 12,
            damage: SmallVec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 28,
            damage: SmallVec::new(),
        });
    }
}

#[derive(Debug)]
pub enum Return {
    Alive,
    Dead,
    Phase,
}

#[derive(Debug)]
pub struct Zulrah {
    rot: Rotation,
    phase: Phase,
    hitpoints: u16,
    time: u16,
    kill: Vec<Phase>,
}

impl fmt::Display for Zulrah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_dead() {
            write!(
                f,
                "{}\n    Phase: {}\n    HP: {}, Ticks: {}\n",
                self.rot.name, self.phase, self.hitpoints, self.time
            )
        } else {
            let res = write!(f, "{}\n", self.rot.name);
            for phase in &self.kill {
                write!(f, "{}\n", phase)?;
            }
            res
        }
    }
}

impl Zulrah {
    pub fn new() -> Zulrah {
        Zulrah::new_rot(Rot::Southgreen)
    }

    pub fn new_rot(rot: Rot) -> Zulrah {
        let mut rot = Rotation::new(rot);
        let phase = rot.next_phase();
        Zulrah {
            rot,
            phase,
            hitpoints: 500,
            time: 0,
            kill: Vec::new(),
        }
    }

    pub fn damage(&mut self, damage: u16, ticks: u16) -> Return {
        assert!(self.hitpoints > 0);

        if ticks >= self.phase.time {
            self.time += self.phase.time;
            self.phase.time = 0;
        } else {
            self.time += ticks;
            self.phase.time -= ticks;
        }

        if damage >= self.hitpoints {
            self.phase.damage.push(self.hitpoints);
            self.kill.push(self.phase.clone());
            self.hitpoints = 0;
            return Return::Dead;
        }

        self.hitpoints -= damage;
        self.phase.damage.push(damage);

        if self.phase.time == 0 {
            self.time += self.rot.delay;
            self.kill.push(self.phase.clone());
            self.phase = self.rot.next_phase();
            Return::Phase
        } else {
            Return::Alive
        }
    }

    pub fn is_dead(&self) -> bool {
        self.hitpoints == 0
    }

    pub fn ttk(&mut self) -> u16 {
        self.time
    }

    pub fn form(&self) -> Form {
        self.phase.form
    }
}

/*
    pub fn test_phases(&mut self) {
        for _ in 0..50 {
            println!("{}", self.rot.next_phase());
        }
    }
*/
