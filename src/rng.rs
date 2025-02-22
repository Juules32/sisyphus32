pub struct RandomNumberGenerator {
    state: u32
}

impl RandomNumberGenerator {
    pub fn new(seed: u32) -> RandomNumberGenerator {
        RandomNumberGenerator { state: seed }
    }

    pub fn generate_u32(&mut self) -> u32 {
        self.state ^= self.state << 13; 
        self.state ^= self.state >> 17; 
        self.state ^= self.state << 5;
        self.state
    }

    pub fn generate_u64(&mut self) -> u64 {
        let [a, b, c, d] = std::array::from_fn(|_| (self.generate_u32() & 0xFFFF) as u64);
        a | (b << 16) | (c << 32) | (d << 48)
    }

    pub fn generate_sparse_u64(&mut self) -> u64 {
        self.generate_u64() & self.generate_u64() & self.generate_u64()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self { state: 1804289383 }
    }
}
