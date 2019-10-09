use super::common::{Benchmark, CPU_MHZ};

const LOCAL_SCALE_FACTOR: usize = 5;

pub struct CubicBench {
    soln_cnt0: i32,
    soln_cnt1: i32,
    res0: [f64; 3],
    res1: f64,
}

impl CubicBench {
    pub fn new() -> CubicBench {
        CubicBench {
            soln_cnt0: 0,
            soln_cnt1: 0,
            res0: [0.0; 3],
            res1: 0.0,
        }
    }
}

impl Benchmark for CubicBench {
    fn initialise_benchmark(&mut self) {
        println!("cubic benchmark starts.");
    }

    fn verify_benchmark(&mut self) -> bool {
        (3 == self.soln_cnt0) && ((2.0 - self.res0[0]).abs() < 1.0e-10) &&
         ((6.0 - self.res0[1]).abs() < 1.0e-10) && ((2.5 - self.res0[2]).abs() < 1.0e-10) &&
         (1 == self.soln_cnt1) && ((2.5 - self.res1).abs() < 1.0e-10)
    }

    fn benchmark(&mut self) {
        self.benchmark_body((LOCAL_SCALE_FACTOR * CPU_MHZ) as i32);
    }

    fn benchmark_body(&mut self, rpt: i32) {
        for _ in 0..rpt {
            let (a1, b1, c1, d1): (f64, f64, f64, f64) = (1.0, -10.5, 32.0, -30.0);
            let (a2, b2, c2, d2): (f64, f64, f64, f64) = (1.0 , -4.5, 17.0, -30.0);
            let (a3, b3, c3, d3): (f64, f64, f64, f64) = (1.0, -3.5, 22.0, -31.0);
            let (a4, b4, c4, d4): (f64, f64, f64, f64) = (1.0, -13.7, 1.0, -35.0);

            let mut solutions: i32 = 0;
            let mut output = [0.0; 48];

            solve_cubic(a1, b1, c1, d1, &mut solutions, &mut output);
            self.soln_cnt0 = solutions;
            self.res0.copy_from_slice(&output[0..3]);

            solve_cubic(a2, b2, c2, d2, &mut solutions, &mut output);
            self.soln_cnt1 = solutions;
            self.res1 = output[0];

            solve_cubic(a3, b3, c3, d3, &mut solutions, &mut output);
            solve_cubic(a4, b4, c4, d4, &mut solutions, &mut output);

            for i in &[1.0, 2.0] {
                for j in &[10.0, 9.0] {
                    for k in &[5.0, 5.5] {
                        for l in &[-1.0, -2.0] {
                            solve_cubic(*i, *j, *k, *l, &mut solutions, &mut output);
                        }
                    }
                }
            }
        }
    }
}

fn solve_cubic(a: f64, b: f64, c: f64, d: f64, solutions: &mut i32, x: &mut [f64]) {
    let a1 = b / a;
    let a2 = c / a;
    let a3 = d / a;
    let Q = (a1 * a1 - 3.0 * a2) / 9.0;
    let R = (2.0 * a1 * a1 * a1 - 9.0 * a1 * a2 + 27.0 * a3) / 54.0;
    let R2_Q3 = R * R - Q * Q * Q;

    if R2_Q3 <= 0.0 {
        *solutions = 3;
        let theta = (R / (Q * Q * Q).sqrt()).acos();
        x[0] = -2.0 * Q.sqrt() * (theta / 3.0).cos() - a1 / 3.0;
        x[1] = -2.0 * Q.sqrt() * ((theta + 2.0 * std::f64::consts::PI) / 3.0).cos() - a1 / 3.0;
        x[2] = -2.0 * Q.sqrt() * ((theta + 4.0 * std::f64::consts::PI) / 3.0).cos() - a1 / 3.0;
    } else {
        *solutions = 1;
        x[0] = (R2_Q3.sqrt() + R.abs()).powf(1.0 / 3.0);
        x[0] += Q / x[0];
        x[0] *= if R < 0.0 { 1.0 } else { -1.0 };
        x[0] -= a1 / 3.0;
    }
}
