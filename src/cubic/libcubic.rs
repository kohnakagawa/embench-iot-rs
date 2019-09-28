fn solve_cubic(a: f64, b: f64, c: f64, d: f64, solutions: &mut i32, x: &mut [f64]) {
    let a1 = b / a; // NOTE: Rust で x87 命令を出すのってできるの？ (オリジナル実装だと long double になっている)
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

fn benchmark_body(rpt: usize) {
    for _ in 0..rpt {
        let (mut a1, mut b1, mut c1, mut d1): (f64, f64, f64, f64) = (1.0, -10.5, 32.0, -30.0);
        let (mut a2, mut b2, mut c2, mut d2): (f64, f64, f64, f64) = (1.0 , -4.5, 17.0, -30.0);
        let (mut a3, mut b3, mut c3, mut d3): (f64, f64, f64, f64) = (1.0, -3.5, 22.0, -31.0);
        let (mut a4, mut b4, mut c4, mut d4): (f64, f64, f64, f64) = (1.0, -13.7, 1.0, -35.0);

        let mut solutions: i32 = 0;
        let mut output = [0.0; 48];

        solve_cubic(a1, b1, c1, d1, &mut solutions, &mut output);

        solve_cubic(a2, b2, c2, d2, &mut solutions, &mut output);

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