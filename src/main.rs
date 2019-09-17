extern crate chrono;
extern crate rand;

mod random;

use log::{debug, info, trace};
use random::Random;
use random::RandomTrait;

static SIZE: u8 = 10;
static TOTAL_RUNS: u32 = 100_000;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f %z"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to initialize logging");
    debug!("Logging initialized");

    let mut rnd = Random::new();

    info!("Computing average for {} runs...", TOTAL_RUNS);
    let (average, _): (f64, u64) = (0..TOTAL_RUNS)
        .map(|_| frog_run(&mut rnd, SIZE))
        .map(u64::from)
        .fold((0., 0), |(average, count), x| {
            (
                (average * count as f64 + x as f64) / (count as f64 + 1.),
                count + 1,
            )
        });
    info!("Average for {} runs: {}", TOTAL_RUNS, average);
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

    trace!("frog_run took {} hops", number_hops);
    number_hops
}

#[cfg(test)]
mod test {
    use super::*;
    use mockers::matchers::ANY;
    use mockers::Scenario;

    #[test]
    fn test_frog_run_random_10() {
        let scenario = Scenario::new();
        let (mut cond, cond_handle) = scenario.create_mock_for::<dyn RandomTrait>();

        scenario.expect(cond_handle.gen_range(1, 10).and_return_clone(10).times(1));

        assert_eq!(1, frog_run(&mut cond, 10));
    }

    #[test]
    fn test_frog_run_random_1() {
        let scenario = Scenario::new();
        let (mut cond, cond_handle) = scenario.create_mock_for::<dyn RandomTrait>();

        scenario.expect(cond_handle.gen_range(1, ANY).and_return_clone(1).times(10));

        assert_eq!(10, frog_run(&mut cond, 10));
    }

    #[test]
    fn test_frog_run_random_2() {
        let scenario = Scenario::new();
        let (mut cond, cond_handle) = scenario.create_mock_for::<dyn RandomTrait>();

        scenario.expect(cond_handle.gen_range(1, ANY).and_return_clone(2).times(5));

        assert_eq!(5, frog_run(&mut cond, 10));
    }

    #[test]
    fn test_frog_run_random_5() {
        let scenario = Scenario::new();
        let (mut cond, cond_handle) = scenario.create_mock_for::<dyn RandomTrait>();

        scenario.expect(cond_handle.gen_range(1, ANY).and_return_clone(5).times(2));

        assert_eq!(2, frog_run(&mut cond, 10));
    }
}
