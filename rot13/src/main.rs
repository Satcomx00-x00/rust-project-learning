// make a random number generator that impletment rngcore to get initial seed for a rot13 cipher

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

#[derive(Debug)]
struct Rot13 {
    rng: XorShiftRng,
}

impl Rot13 {
    fn new() -> Self {
        let mut rng = XorShiftRng::from_entropy();
        Self { rng }
    }
}

impl RngCore for Rot13 {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

fn main() {
    let mut rng = Rot13::new();
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    println!("{:?}", bytes);
}

