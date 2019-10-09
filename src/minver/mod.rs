use super::common::{Benchmark, CPU_MHZ};

const LOCAL_SCALE_FACTOR: usize = 329;

pub struct MinverBench {
    pub a: [[f32; 3]; 3],
    pub a_ref: [[f32; 3]; 3],
    pub b: [[f32; 3]; 3],
    pub c: [[f32; 3]; 3],
    pub d: [[f32; 3]; 3],
    pub det: f32,
}

impl MinverBench {
    pub fn new() -> MinverBench {
        MinverBench{
            a: [[0.0; 3]; 3],
            a_ref: [[3.0, -6.0, 7.0], [9.0, 0.0, -5.0], [5.0, -8.0, 6.0]],
            b: [[-3.0, 0.0, 2.0], [3.0, -2.0, 0.0], [0.0, 2.0, -3.0]],
            c: [[0.0; 3]; 3],
            d: [[0.0; 3]; 3],
            det: 0.0,
        }
    }

    pub fn minver_fabs(n: f32) -> f32 {
        if n >= 0.0 {return n} else {return -n}
    }

    pub fn mmul(&mut self, row_a: usize, col_a: usize, row_b: usize, col_b: usize) -> i32 {
        let row_c = row_a;
        let col_c = col_b;

        if row_c < 1 || row_b < 1 || col_c < 1 || col_a != row_b {
            return 999;
        }

        for i in 0..row_c {
            for j in 0..col_c {
                let mut w: f32 = 0.0;
                for k in 0..row_b {
                    w += self.a[i][k] * self.b[k][j];
                }
                self.c[i][j] = w;
            }
        }
        0
    }

    pub fn minver(&mut self, row: usize, col: usize, eps: f32) -> i32 {
        let mut work = [0; 500];
        let (mut r, mut iw, mut u, mut v): (usize, usize, usize, usize) = (0, 0, 0, 0);
        let (mut w, mut wmax, mut pivot, mut api, mut w1): (f32, f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0, 1.0);

        if row < 2 || row > 500 || eps <= 0.0 {
            return 999;
        }

        for i in 0..row {
            work[i] = i;
        }

        for k in 0..row {
            wmax = 0.0;
            for i in k..row {
                w = MinverBench::minver_fabs(self.a[i][k]);
                if w > wmax {
                    wmax = w;
                    r = i;
                }
            }
            pivot = self.a[r][k];
            api = MinverBench::minver_fabs(pivot);
            if api <= eps {
                self.det = w1;
                return 1;
            }
            w1 *= pivot;
            u = k * col;
            v = r * col;
            if r != k {
                w1 = -w;
                iw = work[k];
                work[k] = work[r];
                work[r] = iw;
                for j in 0..row {
                    w = self.a[k][j];
                    self.a[k][j] = self.a[r][j];
                    self.a[r][j] = w;
                }
            }
            for i in 0..row {
                self.a[k][i] /= pivot;
            }
            for i in 0..row {
                if i != k {
                    v = i * col;
                    w = self.a[i][k];
                    if w != 0.0 {
                        for j in 0..row {
                            if j != k {
                                self.a[i][j] -= w * self.a[k][j];
                            }
                        }
                        self.a[i][k] = -w / pivot;
                    }
                }
            }
            self.a[k][k] = 1.0 / pivot;
        }

        for i in 0..row {
            loop {
                let k = work[i];
                if k == i {
                    break
                }
                iw = work[k];
                work[k] = work[i];
                work[i] = iw;
                for j in 0..row {
                    u = j * col;
                    w = self.a[k][i];
                    self.a[k][i] = self.a[k][k];
                    self.a[k][k] = w;
                }
            }
        }
        self.det = w1;
        0
    }
}


impl Benchmark for MinverBench {
    fn initialise_benchmark(&mut self) {
        println!("Minver benchmark starts.");
    }

    fn warm_caches(&mut self, heat: i32) {
        self.benchmark_body(heat);
    }

    fn benchmark(&mut self) {
        self.benchmark_body((LOCAL_SCALE_FACTOR * CPU_MHZ) as i32);
    }

    fn benchmark_body(&mut self, rpt: i32) {
        const EPS: f32 = 1.0e-6;
        for _ in 0..rpt {
            self.a.copy_from_slice(&self.a_ref);
            self.minver(3, 3, EPS);
            self.d.copy_from_slice(&self.a);
            self.a.copy_from_slice(&self.a_ref);
            self.mmul(3, 3, 3, 3);
        }
    }

    fn verify_benchmark(&mut self) -> bool {
        const EPS: f32 = 1.0e-6;
        const C_EXP: [[f32; 3]; 3] = [
            [-27.0, 26.0, -15.0],
            [-27.0, -10.0, 33.0],
            [-39.0, 28.0, -8.0]
        ];
        const D_EXP: [[f32; 3]; 3] = [
            [0.133333325, -0.199999958, 0.2666665910],
            [-0.519999862, 0.113333330, 0.5266665220],
            [0.479999840, -0.359999895, 0.0399999917]
        ];

        for i in 0..3 {
            for j in 0..3 {
                if ((self.c[i][j] - C_EXP[i][j]).abs() > EPS) || ((self.d[i][j] - D_EXP[i][j]).abs() > EPS) {
                    return false;
                }
            }
        }
        (self.det + 16.6666718) <= EPS
    }
}