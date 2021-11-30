use randomize::{formulas::f32_half_open_left, RandRangeU32, PCG32};
use zulrah::{Return, Rot, Zulrah};

mod zulrah;

struct DPS {
    ticks: u16,
    max_hit: u16,
    acc: f32,
    form: zulrah::Form,
    rng: RandRangeU32,
}

impl DPS {
    pub fn new(ticks: u16, max_hit: u16, acc: f32, form: zulrah::Form) -> DPS {
        DPS {
            ticks,
            max_hit,
            acc,
            form,
            rng: RandRangeU32::new(0, max_hit as u32),
        }
    }

    pub fn average_hit(&self) -> u16 {
        (self.max_hit as f32 / 2.0 * self.acc) as u16
    }

    pub fn hit(&self, rng_gen: &mut PCG32) -> u16 {
        if f32_half_open_left(rng_gen.next_u32()) < self.acc {
            let mut hit = self.rng.sample(rng_gen) as u16;
            if hit > 50 {
                hit = RandRangeU32::new(45,50).sample(rng_gen) as u16;
            }
            hit
        } else {
            0u16
        }
    }
}

struct Player {
    green: DPS,
    red: DPS,
    blue: DPS,
    rng_gen: PCG32,
}

fn test_kill_zulrah(p: &mut Player, rot: Rot) -> (u32, Zulrah) {
    let mut z = Zulrah::new_rot(rot);

    let mut curr_setup = match z.form() {
        zulrah::Form::Serpentine => &p.green,
        zulrah::Form::Magma => &p.red,
        zulrah::Form::Tanzanite => &p.blue,
    };

    //println!("{}", &z);
    loop {
        let rand_damage = curr_setup.hit(&mut p.rng_gen);
        //println!("rand: {}", rand_damage);
        match z.damage(rand_damage, curr_setup.ticks) {
            Return::Alive => continue,
            Return::Dead => break,
            Return::Phase => (),
        }

        let form = z.form();
        curr_setup = match form {
            zulrah::Form::Serpentine => &p.green,
            zulrah::Form::Magma => &p.red,
            zulrah::Form::Tanzanite => &p.blue,
        };
        //println!("{}", &z);
    }

    if z.is_dead() {
        //print!("{}", &z);
        //println!("It took {} ticks to kill Zulrah", z.ttk());
    }
    (z.ttk() as u32, z)
}

fn wrapper(p: &mut Player, rot: Rot) -> f64 {
    let trials = 100_000;
    let mut sum: u32 = 0;
    let mut fastest: u32 = 1000;
    let mut _fz: Zulrah = Zulrah::new_rot(rot);
    for _ in 0..trials {
        let (time, z) = test_kill_zulrah(p, rot);
        sum += time;
        if time < fastest {
            fastest = time;
            _fz = z;
        }
    }
    let avg = sum as f64 / trials as f64;
    //println!("  It took {} ticks to kill {}", avg, &rot);
    //println!("    Fastest Zulrah: {}", fastest);
    //println!("{}", &_fz);
    avg
}

fn beats_time(p: &mut Player, rot: Rot, trials: usize, time_to_beat: u32) {
    let mut count = 0;
    let mut _fz: Zulrah = Zulrah::new_rot(rot);
    for _ in 0..trials {
        let (time, _z) = test_kill_zulrah(p, rot);
        if time <= time_to_beat {
            count+=1;
        }
    }
    println!("Out of {} trials on rotation {}, {} hit the GM time of {}", trials, rot, count, time_to_beat);
}

fn main() {
    let mage_green = DPS::new(4, 41, 90.84 / 100.0, zulrah::Form::Serpentine);
    let mage_red = DPS::new(4, 41, 69.14 / 100.0, zulrah::Form::Magma);
    let range_blue = DPS::new(2, 32, 63.57 / 100.0, zulrah::Form::Tanzanite);

    let mut buf = [0u8; 8];
    getrandom::getrandom(&mut buf).unwrap();
    let seed = u64::from_ne_bytes(buf);
    let rng_gen = randomize::PCG32::seed(seed, seed);

    let mut p = Player {
        green: mage_green,
        red: mage_red,
        blue: range_blue,
        rng_gen,
    };

    println!("Zulrah Average TTK (1000k)");

    let mut avgs = [0f64;4];
    avgs[0] = wrapper(&mut p, Rot::Southgreen);
    avgs[1] = wrapper(&mut p, Rot::Westgreen);
    avgs[2] = wrapper(&mut p, Rot::Eastgreen);
    avgs[3] = wrapper(&mut p, Rot::Eastblue);

    println!("  It took {:.0} ticks on average to kill Zulrah.",
        avgs.iter().sum::<f64>() / avgs.len() as f64);

    //beats_time(&mut p, Rot::Southgreen, 1_000_000, 90);
    //beats_time(&mut p, Rot::Westgreen, 1_000_000, 90);
    //beats_time(&mut p, Rot::Eastgreen, 1_000_000, 90);
    //beats_time(&mut p, Rot::Eastblue, 1_000_000, 90);
}
