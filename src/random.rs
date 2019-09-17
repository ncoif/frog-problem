#[cfg(test)]
use mockers_derive::mocked;

use log::trace;
use rand::Rng;

#[cfg_attr(test, mocked)]
pub trait RandomTrait {
    fn gen_range(&mut self, low: u8, high: u8) -> u8;
}

pub struct Random {}

impl Random {
    pub fn new() -> Self {
        Random {}
    }
}

impl RandomTrait for Random {
    fn gen_range(&mut self, low: u8, high: u8) -> u8 {
        let rand = if low == high {
            low
        } else {
            rand::thread_rng().gen_range(low, high)
        };

        trace!("gen_range({}, {}) = {}", low, high, rand);

        rand
    }
}
