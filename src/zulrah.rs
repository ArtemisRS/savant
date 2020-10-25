use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub enum PhaseState {
    Start(u8),
    Ongoing(u8),
    End,
}

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
    time: u16,  //ticks
    delay: u16, //ticks
    damage: Vec<u16>,
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

//one full rotation - lasts until Zulrah resets at middle
#[derive(Debug)]
struct Rotation {
    name: String,
    phases: VecDeque<Phase>,
    //time: u8,
    //state: PhaseState,
    //form: Form,
}

impl Rotation {
    //TODO: should randomly pick one of the four rotations
    fn new() -> Rotation {
        let mut phases = VecDeque::new();

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 20,
            delay: 6,
            damage: Vec::new(),
        });

        let name = Rotation::add_phases(&mut phases);
        Rotation { name, phases }
    }

    fn add_phases(phases: &mut VecDeque<Phase>) -> String {
        Rotation::south_green(phases);
        "Rotation 1: South Green".to_string()
    }

    fn next_phase(&mut self) -> Phase {
        let phase = self.phases.pop_front();
        match phase {
            Some(phase) => phase,
            None => {
                Rotation::add_phases(&mut self.phases);
                self.phases.pop_front().unwrap()
            }
        }
    }

    //fn time_in_phase(&self) -> u8 {
    //    self.time
    //}

    ////number of ticks to advance time forward
    //fn advance(&mut self, ticks: u8) -> (PhaseState, Form) {

    //    if let PhaseState::End = self.state {
    //        //this is bad - should never be here
    //        panic!();
    //    }

    //    let rem = match self.state {
    //        PhaseState::Start =>

    //    match self.state {
    //        PhaseState::Start => {
    //            let rem =
    //        },
    //        PhaseState::Ongoing(t) => (),
    //        PhaseState::End => (),
    //        _ => (),
    //    }
    //    (PhaseState::Start, Form::Serpentine)
    //}

    fn south_green(phases: &mut VecDeque<Phase>) {
        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 14,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::Middle,
            time: 11,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 33,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 16,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::West,
            time: 14,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::South,
            time: 22,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Tanzanite,
            loc: Location::South,
            time: 30,
            delay: 6,
            damage: Vec::new(),
        });

        //jad
        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::West,
            time: 42,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Magma,
            loc: Location::Middle,
            time: 15,
            delay: 6,
            damage: Vec::new(),
        });

        phases.push_back(Phase {
            form: Form::Serpentine,
            loc: Location::Middle,
            time: 28,
            delay: 6,
            damage: Vec::new(),
        });
    }

    fn west_green() -> Rotation {
        todo!()
    }

    fn east_green() -> Rotation {
        todo!()
    }

    fn east_blue() -> Rotation {
        todo!()
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
        let mut rot = Rotation::new();
        let phase = rot.next_phase();
        Zulrah {
            rot,
            phase,
            hitpoints: 500,
            time: 0,
            kill: Vec::new(),
        }
    }

    pub fn test_phases(&mut self) {
        for _ in 0..50 {
            println!("{}", self.rot.next_phase());
        }
    }

    pub fn damage(&mut self, damage: u16, ticks: u16) -> Return {
        assert!(self.hitpoints > 0);

        if ticks >= self.phase.time {
            self.time += self.phase.time;
            self.phase.time = 0;
        } else {
            self.time += ticks as u16;
            self.phase.time -= ticks;
        }

        if damage as u16 >= self.hitpoints {
            self.phase.damage.push(self.hitpoints);
            self.kill.push(self.phase.clone());
            self.hitpoints = 0;
            return Return::Dead;
        }

        self.hitpoints -= damage as u16;
        self.phase.damage.push(damage);

        if self.phase.time == 0 {
            self.time += self.phase.delay as u16;
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

    //fn form(self) -> Form {
    //    self.rotation.
}
