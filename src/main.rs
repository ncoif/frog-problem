extern crate rand;

use rand::rngs::ThreadRng;
use rand::Rng;

static SIZE: u8 = 10;
static TOTAL_RUNS: u32 = 100_000;

fn main() {
    let mut random_generator = rand::thread_rng();
    let (average, _): (f64, u64) = (0..TOTAL_RUNS)
        .map(|_| frog_run(SIZE, &mut random_generator))
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
fn frog_run(size: u8, random_generator: &mut ThreadRng) -> u8 {
    let mut remaining_distance = size;

    let mut number_hops = 0;
    while remaining_distance > 0 {
        let next_hop = gen_range(1, remaining_distance, random_generator);
        number_hops += 1;
        remaining_distance -= next_hop;
    }

    number_hops
}

fn gen_range(low: u8, high: u8, random_generator: &mut ThreadRng) -> u8 {
    if low == high {
        low
    } else {
        random_generator.gen_range(low, high)
    }
}
