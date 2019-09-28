struct Body {
    x: [f64; 3],
    fill: f64,
    v: [f64; 3],
    mass: f64,
}

const CPU_MHZ: usize = 1;
const LOCAL_SCALE_FACTOR: usize = 1;
const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;

const BODIES_SIZE: usize = 5;

static mut SOLAR_BODIES: [Body;BODIES_SIZE] = [
    // sun
    Body {
        x: [0.0, 0.0, 0.0],
        fill: 0.0,
        v: [0.0, 0.0, 0.0],
        mass: SOLAR_MASS,
    },
    // jupiter
    Body {
        x: [4.84143144246472090e+00, -1.16032004402742839e+00, -1.03622044471123109e-01],
        fill: 0.0,
        v: [1.66007664274403694e-03 * DAYS_PER_YEAR, 7.69901118419740425e-03 * DAYS_PER_YEAR, -6.90460016972063023e-05 * DAYS_PER_YEAR],
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    },
    // saturn
    Body {
        x: [8.34336671824457987e+00, 4.12479856412430479e+00, -4.03523417114321381e-01],
        fill: 0.0,
        v: [-2.76742510726862411e-03 * DAYS_PER_YEAR, 4.99852801234917238e-03 * DAYS_PER_YEAR, 2.30417297573763929e-05 * DAYS_PER_YEAR],
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    },
    // uranus
    Body {
        x: [1.28943695621391310e+01, -1.51111514016986312e+01, -2.23307578892655734e-01],
        fill: 0.0,
        v: [2.96460137564761618e-03 * DAYS_PER_YEAR, 2.37847173959480950e-03 * DAYS_PER_YEAR, -2.96589568540237556e-05 * DAYS_PER_YEAR],
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    },
    // neptune
    Body {
        x: [1.53796971148509165e+01, -2.59193146099879641e+01, 1.79258772950371181e-01],
        fill: 0.0,
        v: [2.68067772490389322e-03 * DAYS_PER_YEAR, 1.62824170038242295e-03 * DAYS_PER_YEAR, -9.51592254519715870e-05 * DAYS_PER_YEAR],
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    }
];

fn offset_momentum(bodies: &mut [Body], nbodies: usize) {
    for i in 0..nbodies {
        for k in 0..3 {
            bodies[i].v[k] -= bodies[i].v[k] * bodies[i].mass / SOLAR_MASS;
        }
    }
}

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

fn warm_caches(heat: usize) {
    benchmark_body(heat)
}

fn benchmark() {
    benchmark_body(LOCAL_SCALE_FACTOR * CPU_MHZ)
}

fn verify_benchmark() -> u32 {
    const EXPECTED: [Body;BODIES_SIZE] = [
        // sun
        Body {
            x: [0.0, 0.0, 0.0],
            fill: 0.0,
            v: [-0.000387663407198742665776131088862, -0.0032753590371765706722173572274, 2.39357340800030020670947916717e-05],
            mass: 39.4784176043574319692197605036,
        },
        // jupiter
        Body {
            x: [4.84143144246472090230781759601, -1.16032004402742838777840006514, -0.103622044471123109232735259866],
            fill: 0.0,
            v: [0.606326392995832019749968821998, 2.81198684491626016423992950877, -0.0252183616598876288172892401462],
            mass: 0.0376936748703894930478952574049,
        },
        // saturn
        Body {
            x: [8.34336671824457987156620220048, 4.1247985641243047894022311084, -0.403523417114321381049535375496],
            fill: 0.0,
            v: [-1.01077434617879236000703713216, 1.82566237123041186229954746523, 0.00841576137658415351916474378413],
            mass: 0.0112863261319687668143840753032,
        },
        // uranus
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

    unsafe {
        for i in 0..BODIES_SIZE {
            for j in 0..3 {
                if SOLAR_BODIES[i].x[j] != EXPECTED[i].x[j] {
                    println!("x: {} {}", SOLAR_BODIES[i].x[j], EXPECTED[i].x[j]);
                    return 0;
                }
                if SOLAR_BODIES[i].v[j] != EXPECTED[i].v[j] {
                    println!("v: {} {}", SOLAR_BODIES[i].v[j], EXPECTED[i].v[j]);
                    return 0;
                }
            }
            if SOLAR_BODIES[i].mass != EXPECTED[i].mass {
                println!("{} {}", SOLAR_BODIES[i].mass, EXPECTED[i].mass);
                return 0;
            }
        }
        1
    }
}

#[inline(never)]
fn benchmark_body(rpt: usize) {
    unsafe {
        for _ in 0..rpt {
            offset_momentum(&mut SOLAR_BODIES, BODIES_SIZE);
            for _ in 0..100 {
                bodies_energy(&mut SOLAR_BODIES, BODIES_SIZE);
            }
        }
    }
}

fn main() {
    benchmark();
    let ret = verify_benchmark();
    println!("{}", ret);
}