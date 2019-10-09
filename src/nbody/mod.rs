use super::common::{Benchmark, CPU_MHZ};

const LOCAL_SCALE_FACTOR: usize = 1;
const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_SIZE: usize = 5;

struct Body {
    x: [f64; 3],
    fill: f64,
    v: [f64; 3],
    mass: f64,
}

impl Body {
    pub fn new(x: &[f64; 3], v:&[f64; 3], mass: f64) -> Body {
        Body {
            x: *x,
            fill: 0.0,
            v: *v,
            mass: mass,
        }
    }
}

impl NbodyBench {
    pub fn new() -> NbodyBench {
        NbodyBench {
            solar_bodies: [
                Body::new(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0], SOLAR_MASS),
                Body::new(
                    &[4.84143144246472090e+00, -1.16032004402742839e+00, -1.03622044471123109e-01],
                    &[1.66007664274403694e-03 * DAYS_PER_YEAR, 7.69901118419740425e-03 * DAYS_PER_YEAR, -6.90460016972063023e-05 * DAYS_PER_YEAR],
                    9.54791938424326609e-04 * SOLAR_MASS,
                ),
                Body::new(
                    &[8.34336671824457987e+00, 4.12479856412430479e+00, -4.03523417114321381e-01],
                    &[-2.76742510726862411e-03 * DAYS_PER_YEAR, 4.99852801234917238e-03 * DAYS_PER_YEAR, 2.30417297573763929e-05 * DAYS_PER_YEAR],
                    2.85885980666130812e-04 * SOLAR_MASS,
                ),
                Body::new(
                    &[1.28943695621391310e+01, -1.51111514016986312e+01, -2.23307578892655734e-01],
                    &[2.96460137564761618e-03 * DAYS_PER_YEAR, 2.37847173959480950e-03 * DAYS_PER_YEAR, -2.96589568540237556e-05 * DAYS_PER_YEAR],
                    4.36624404335156298e-05 * SOLAR_MASS,
                ),
                Body::new(
                    &[1.53796971148509165e+01, -2.59193146099879641e+01, 1.79258772950371181e-01],
                    &[2.68067772490389322e-03 * DAYS_PER_YEAR, 1.62824170038242295e-03 * DAYS_PER_YEAR, -9.51592254519715870e-05 * DAYS_PER_YEAR],
                    5.15138902046611451e-05 * SOLAR_MASS,
                ),
            ]
        }
    }
}

pub struct NbodyBench {
    solar_bodies: [Body; BODIES_SIZE]
}

impl Benchmark for NbodyBench {
    fn initialise_benchmark(&mut self) {
        println!("nbody benchmark starts.");
    }

    fn benchmark_body(&mut self, rpt: i32) {
        for _ in 0..rpt {
            offset_momentum(&mut self.solar_bodies, BODIES_SIZE);
            for _ in 0..100 {
                bodies_energy(&mut self.solar_bodies, BODIES_SIZE);
            }
        }
    }

    fn benchmark(&mut self) {
        self.benchmark_body((LOCAL_SCALE_FACTOR * CPU_MHZ) as i32);
    }

    fn verify_benchmark(&mut self) -> bool {
        const EXPECTED: [Body;BODIES_SIZE] = [
            Body {
                x: [0.0, 0.0, 0.0],
                fill: 0.0,
                v: [-0.000387663407198742665776131088862, -0.0032753590371765706722173572274, 2.39357340800030020670947916717e-05],
                mass: 39.4784176043574319692197605036,
            },
            Body {
                x: [4.84143144246472090230781759601, -1.16032004402742838777840006514, -0.103622044471123109232735259866],
                fill: 0.0,
                v: [0.606326392995832019749968821998, 2.81198684491626016423992950877, -0.0252183616598876288172892401462],
                mass: 0.0376936748703894930478952574049,
            },
            Body {
                x: [8.34336671824457987156620220048, 4.1247985641243047894022311084, -0.403523417114321381049535375496],
                fill: 0.0,
                v: [-1.01077434617879236000703713216, 1.82566237123041186229954746523, 0.00841576137658415351916474378413],
                mass: 0.0112863261319687668143840753032,
            },
            Body {
                x: [12.8943695621391309913406075793, -15.1111514016986312469725817209, -0.223307578892655733682204299839],
                fill: 0.0,
                v: [1.08279100644153536414648897335, 0.868713018169608219842814378353, -0.0108326374013636358983880825235],
                mass: 0.0017237240570597111687795033319,
            },
            // neptune
            Body {
                x: [15.3796971148509165061568637611, -25.9193146099879641042207367718, 0.179258772950371181309492385481],
                fill: 0.0,
                v: [0.979090732243897976516677772452, 0.594698998647676169149178804219, -0.0347559555040781037460462243871],
                mass: 0.00203368686992463042206846779436,
            }
        ];

        for i in 0..BODIES_SIZE {
            for j in 0..3 {
                if self.solar_bodies[i].x[j] != EXPECTED[i].x[j] {
                    println!("x: {} {}", self.solar_bodies[i].x[j], EXPECTED[i].x[j]);
                    return false;
                }
                if self.solar_bodies[i].v[j] != EXPECTED[i].v[j] {
                    println!("v: {} {}", self.solar_bodies[i].v[j], EXPECTED[i].v[j]);
                    return false;
                }
            }
            if self.solar_bodies[i].mass != EXPECTED[i].mass {
                println!("{} {}", self.solar_bodies[i].mass, EXPECTED[i].mass);
                return false;
            }
        }
        true
    }
}

#[inline(never)]
fn offset_momentum(bodies: &mut [Body], nbodies: usize) {
    for i in 0..nbodies {
        for k in 0..3 {
            bodies[0].v[k] -= bodies[i].v[k] * bodies[i].mass / SOLAR_MASS;
        }
    }
}

#[inline(never)]
fn bodies_energy(bodies: &[Body], nbodies: usize) -> f64 {
    let mut e = 0.0;
    for i in 0..nbodies {
        e += bodies[i].mass * (bodies[i].v[0] * bodies[i].v[0]
            + bodies[i].v[1] * bodies[i].v[1]
            + bodies[i].v[2] * bodies[i].v[2]) / 2.;

        for j in i+1..nbodies {
            let dx = bodies[i].x[0] - bodies[j].x[0];
            let dy = bodies[i].x[1] - bodies[j].x[1];
            let dz = bodies[i].x[2] - bodies[j].x[2];
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            e -= (bodies[i].mass * bodies[j].mass) / distance;
        }
    }
    e
}
