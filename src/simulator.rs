use crate::zulrah::{Return, Rot, Zulrah};
use crate::zulrah;
use randomize::{formulas::f32_half_open_left, RandRangeU32, PCG32};

struct DPS {
    ticks: u16,
    max_hit: u16,
    acc: f32,
    rng: RandRangeU32,
}

impl DPS {
    pub fn new(ticks: u16, max_hit: u16, acc: f32) -> DPS {
        DPS {
            ticks,
            max_hit,
            acc,
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

fn average_kill_time(p: &mut Player, rot: Rot) -> f64 {
    let trials = 1_000_000;
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

pub fn main() {
    let cbow = DPS::new(4, 38, 79.86 / 100.0);

    //elite void, no ring
    let bp_addy = DPS::new(2, 31, 62.71 / 100.0);
    let bp_rune = DPS::new(2, 32, 63.57 / 100.0);
    let bp_drag = DPS::new(2, 33, 64.19 / 100.0);

    //full arma, no ring
    let nerf_bp_addy = DPS::new(2, 24, 67.00 / 100.0);
    let nerf_bp_rune = DPS::new(2, 26, 67.61 / 100.0);
    let nerf_bp_drag = DPS::new(2, 28, 68.2 / 100.0);
    let nerf_bp_drag_void = DPS::new(2, 32, 55.98 / 100.0);

    //elite void, no ring
    let acb_drag = DPS::new(5, 49, 70.79 / 100.0);
    let acb_rune = DPS::new(5, 47, 70.79 / 100.0);
    let kbow_void = DPS::new(3, 32, 65.36 / 100.0);
    let msb_ame = DPS::new(3, 32, 63.57 / 100.0);
    let msb_rune = DPS::new(3, 31, 63.57 / 100.0);

    //full arma, no ring
    let tbow_drag = DPS::new(5, 73, 79.61 / 100.0);
    let tbow_ame = DPS::new(5, 70, 79.61 / 100.0);
    let tbow_rune = DPS::new(5, 68, 79.61 / 100.0);
    let kbow_arma = DPS::new(3, 29, 73.01 / 100.0);
    let msb_ame_arma = DPS::new(3, 29, 72.03/ 100.0);
    let msb_rune_arma = DPS::new(3, 27, 72.03 / 100.0);

    //has a 25% of doing a second a hit of up to half the max hit
    //added temp code to test that has now been removed
    let kbow_damned = DPS::new(3, 27, 71.46 / 100.0);

    //toxic trident/mage's book/no ring/full ancestral
    let max_mage_green = DPS::new(4, 41, 90.84 / 100.0);
    let max_mage_red = DPS::new(4, 41, 69.14 / 100.0);

    //toxic trident/arma helm/ancestral top/ah bot/seers/mage's book
    let sim_green_alt = DPS::new(4, 40, 90.62 / 100.0);
    let sim_red_alt = DPS::new(4, 40, 68.42 / 100.0);

    //toxic trident/no helm/ancestral top/ah bot/seers/mage's book
    let sim_green = DPS::new(4, 40, 90.84 / 100.0);
    let sim_red = DPS::new(4, 40, 69.14 / 100.0);

    //toxic trident/slay helm/ancestral top/ah bot/seers/mage's book
    let sim_green_slay = DPS::new(4, 40, 90.96 / 100.0);
    let sim_red_slay = DPS::new(4, 40, 69.56 / 100.0);

    //toxic trident/farseer helm/ancestral top/ah bot/seers/mage's book
    let sim_green_helm = DPS::new(4, 40, 91.08 / 100.0);
    let sim_red_helm = DPS::new(4, 40, 69.97 / 100.0);


    //current numbers
    //bp/addy darts/arma/no helm/archers ring
    let sim_arma_alt = DPS::new(2, 27, 71.34 / 100.0);
    let sim_arma_alt = DPS::new(2, 28, 71.81 / 100.0);

    //bp/addy darts/arma/slay helm/archers ring
    let sim_arma_slay = DPS::new(2, 27, 71.69 / 100.0);
    let sim_arma_slay = DPS::new(2, 28, 72.14 / 100.0);

    //bp/addy darts/arma/arma helm/archers ring
    let sim_arma = DPS::new(2, 27, 72.47 / 100.0);
    let sim_arma = DPS::new(2, 28, 72.90 / 100.0);

    //bp/addy darts/void/archers ring
    let sim_void = DPS::new(2, 31, 64.39 / 100.0);
    let sim_void = DPS::new(2, 32, 65.17 / 100.0);


    //max with crystal armour, ros
    let green_fbow = DPS::new(4, 46, 67.87 / 100.0);
    let red_fbow = DPS::new(4, 46, 24.37 / 100.0);
    let blue_fbow = DPS::new(4, 46, 81.96 / 100.0);


    let mage_green = green_fbow;
    let mage_red = red_fbow;
    let range_blue = blue_fbow;

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

    let trials = 1_000_000;
    println!("Zulrah Average TTK ({}k trials)", trials/1000);

    let mut avgs = [0f64;4];
    avgs[0] = average_kill_time(&mut p, Rot::Southgreen);
    avgs[1] = average_kill_time(&mut p, Rot::Westgreen);
    avgs[2] = average_kill_time(&mut p, Rot::Eastgreen);
    avgs[3] = average_kill_time(&mut p, Rot::Eastblue);

    println!("  It took {:.0} ticks on average to kill Zulrah.",
        avgs.iter().sum::<f64>() / avgs.len() as f64);

    let gm_time = 90;
    //beats_time(&mut p, Rot::Southgreen, trials, gm_time);
    //beats_time(&mut p, Rot::Westgreen, trials, gm_time);
    //beats_time(&mut p, Rot::Eastgreen, trials, gm_time);
    //beats_time(&mut p, Rot::Eastblue, trials, gm_time);
}
