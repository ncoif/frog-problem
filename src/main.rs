extern crate rand;

mod random;

use random::Random;
use random::RandomTrait;

static SIZE: u8 = 10;
static TOTAL_RUNS: u32 = 100_000;

fn main() {
    let mut rnd = Random::new();

    let (average, _): (f64, u64) = (0..TOTAL_RUNS)
        .map(|_| frog_run(&mut rnd, SIZE))
        .map(u64::from)
        .fold((0., 0), |(average, count), x| {
            (
                (average * count as f64 + x as f64) / (count as f64 + 1.),
                count + 1,
            )
        });
    println!("Average for {} runs: {}", TOTAL_RUNS, average);
}

// return the number of hops for a single run
fn frog_run(rnd: &mut dyn RandomTrait, size: u8) -> u8 {
    let mut remaining_distance = size;

    let mut number_hops = 0;
    while remaining_distance > 0 {
        let next_hop = rnd.gen_range(1, remaining_distance);
        number_hops += 1;
        remaining_distance -= next_hop;
    }

    number_hops
}
