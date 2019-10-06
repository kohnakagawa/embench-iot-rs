pub trait Benchmark {
    fn initialise_benchmark(&mut self) {}
    fn warm_caches(&mut self, heat: i32) {
        self.benchmark_body(heat);
    }
    fn benchmark(&mut self);
    fn benchmark_body(&mut self, rpt: i32);
    fn verify_benchmark(&mut self) -> bool;
}

pub const CPU_MHZ: usize = 1;
pub const WARMUP_HEAT: i32 = 10;