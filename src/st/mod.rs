use super::common::{Benchmark, CPU_MHZ};

const LOCAL_SCALE_FACTOR: usize = 11;
const MAX: usize = 100;

pub struct StBench {
    pub a: [f64; MAX],
    pub b: [f64; MAX],
    pub suma: f64,
    pub sumb: f64,
    pub coef: f64,
    pub rgen: RandomGenerator,
}

pub struct RandomGenerator {
    seed: i32,
}

impl RandomGenerator {
    pub fn new() -> RandomGenerator {
        RandomGenerator {
            seed: 0
        }
    }

    pub fn integer(&mut self) -> i32 {
        self.seed = (self.seed * 133 + 81) % 8095;
        self.seed
    }

    pub fn init_seed(&mut self) {
        self.seed = 0;
    }
}

impl StBench {
    pub fn new() -> StBench {
        StBench{
            a: [0.0; MAX],
            b: [0.0; MAX],
            suma: 0.0,
            sumb: 0.0,
            coef: 0.0,
            rgen: RandomGenerator::new(),
        }
    }

    fn initialize(&mut self) {
        for i in 0..MAX {
            self.a[i] = i as f64 + self.rgen.integer() as f64 / 8095.0;
        }
        for i in 0..MAX {
            self.b[i] = i as f64 + self.rgen.integer() as f64 / 8095.0;
        }
    }

}

fn calc_sum_mean(array: &mut [f64], sum: &mut f64, mean: &mut f64) {
    *sum = 0.0;
    for i in 0..MAX {
        *sum += array[i];
    }
    *mean = *sum / MAX as f64;
}

fn calc_var_stddev(array: &mut [f64], mean: f64, var: &mut f64, stddev: &mut f64) {
    let mut diffs = 0.0;
    for i in 0..MAX {
        diffs += (array[i] - mean) * (array[i] - mean);
    }
    *var = diffs / (MAX as f64);
    *stddev = (*var).sqrt();
}

fn calc_lin_corr_coef(array_a: &mut [f64], array_b: &mut [f64], mean_a: f64, mean_b: f64) -> f64 {
    let mut numerator = 0.0;
    let (mut a_term, mut b_term): (f64, f64) = (0.0, 0.0);
    for i in 0..MAX {
        numerator += (array_a[i] - mean_a) * (array_b[i] - mean_b);
        a_term += (array_a[i] - mean_a) * (array_a[i] - mean_a);
        b_term += (array_b[i] - mean_b) * (array_b[i] - mean_b);
    }

    numerator / (a_term.sqrt() * b_term.sqrt())
}

impl Benchmark for StBench {
    fn initialise_benchmark(&mut self) {
        println!("st benchmark starts.");
    }

    fn warm_caches(&mut self, heat: i32) {
        self.benchmark_body(heat);
    }

    fn benchmark(&mut self) {
        self.benchmark_body((LOCAL_SCALE_FACTOR * CPU_MHZ) as i32);
    }

    fn benchmark_body(&mut self, rpt: i32) {
        for _ in 0..rpt {
            let (mut mean_a, mut mean_b, mut var_a, mut var_b, mut stddev_a, mut stddev_b) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

            self.rgen.init_seed();

            self.initialize();

            calc_sum_mean(&mut self.a, &mut self.suma, &mut mean_a);
            calc_var_stddev(&mut self.a, mean_a, &mut var_a, &mut stddev_a);

            calc_sum_mean(&mut self.b, &mut self.sumb, &mut mean_b);
            calc_var_stddev(&mut self.b, mean_b, &mut var_b, &mut stddev_b);

            self.coef = calc_lin_corr_coef(&mut self.a, &mut self.b, mean_a, mean_b);
        }
    }

    fn verify_benchmark(&mut self) -> bool {
        let exp_suma = 4999.00247066090196;
        let exp_sumb = 4996.84311303273534;
        let exp_coef = 0.999900054853619324;

        ((self.suma - exp_suma).abs() < 1.0e-13)
            && ((self.sumb - exp_sumb).abs() < 1.0e-13) && ((self.coef - exp_coef) < 1.0e-17)
    }
}