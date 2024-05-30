
pub struct Rng(u64);

impl Rng {
    /// Seed the rng with a static value to repro
    pub fn new() -> Self {
        let mut seed_val: u64 = 0;
        #[cfg(target_pointer_width = "64")]
        {unsafe {seed_val = core::arch::x86_64::_rdtsc() } }

        #[cfg(target_pointer_width = "32")]
        {unsafe {seed_val = core::arch::x86::_rdtsc() } }
        Rng(seed_val)
    }

    /// Use own seed value to seed the rng
    pub fn new_with_seed(seed: u64) -> Self {
        Rng(seed)
    }

    /// Simple XOR rng
    pub fn rand(&mut self) -> u64 {
        let val = self.0;
        self.0 ^= self.0 << 10;
        self.0 ^= self.0 >> 14;
        self.0 ^= self.0 << 3;
        val 
    }
}

