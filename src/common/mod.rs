pub trait Benchmark {
    fn initialise_benchmark(&mut self) {}
    fn warm_caches(&mut self, heat: i32) {
        self.benchmark_body(heat);
    }
    fn benchmark(&mut self);
    fn benchmark_body(&mut self, rpt: i32);
    fn verify_benchmark(&mut self) -> bool;
}

pub const CPU_MHZ: usize = 1700;
pub const WARMUP_HEAT: i32 = 1;

pub struct BeebsRandomGenerator {
    seed: i64,
}

impl BeebsRandomGenerator {
    pub fn new() -> BeebsRandomGenerator {
        BeebsRandomGenerator {
            seed: 0,
        }
    }

    pub fn rand_beebs(&mut self) -> i32 {
        self.seed = (self.seed * 1103515245 + 12345) & ((1 << 31) - 1);
        (self.seed >> 16) as i32
    }

    pub fn srand_beebs(&mut self, new_seed: i64) {
        self.seed = new_seed;
    }
}
