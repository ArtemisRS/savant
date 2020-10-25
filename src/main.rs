use randomize::{formulas::f32_half_open_left, RandRangeU32, PCG32};
use zulrah::{Return, Zulrah};

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
            self.rng.sample(rng_gen) as u16
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


fn do_it(p: &mut Player, z: &mut Zulrah) -> u16 {

    0u16
}


fn main() {
    let mut z = Zulrah::new();
    let mage_green = DPS::new(4, 41, 90.84 / 100.0, zulrah::Form::Serpentine);
    let mage_red = DPS::new(4, 41, 69.14 / 100.0, zulrah::Form::Magma);
    let range_blue = DPS::new(2, 32, 63.57 / 100.0, zulrah::Form::Tanzanite);

    let mut buf = [0u8; 8];
    getrandom::getrandom(&mut buf).unwrap();
    let seed = u64::from_ne_bytes(buf);
    let mut rng_gen = randomize::PCG32::seed(seed, seed);

    let mut p = Player {
        green: mage_green,
        red: mage_red,
        blue: range_blue,
        rng_gen,
    };

    let mut curr_setup: &DPS;

    let form = z.form();
    curr_setup = match form {
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
        print!("{}", &z);
        println!("It took {} ticks to kill Zulrah", z.ttk());
    }
}
