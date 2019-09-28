fn mulul64(u: u64, v: u64, whi: &mut u64, wlo: &mut u64) {
  let u1 = u >> 32;
  let u0 = u & 0xFFFFFFFF;
  let v1 = v >> 32;
  let v0 = v & 0xFFFFFFFF;

  let mut t = u0 * v0;
  let w0 = t & 0xFFFFFFFF;
  let mut k = t >> 32;

  t = u1 * v0 + k;
  let w1 = t & 0xFFFFFFFF;
  let w2 = t >> 32;

  t = u0 * v1 + w1;
  k = t >> 32;

  *wlo = (t << 32) + w0;
  *whi = u1 * v1 + w2 + k;
}

fn modul64(mut x: u64, mut y: u64, z: u64) -> u64 {
    for i in 1..=64 {
        let t = (x as i64) >> 63;
        x = (x << 1) | (y >> 63);
        y = y << 1;
        if (x | t as u64) >= z { // NOTE: cast の仕方が C 言語のものとは違う可能性がある
            x = x - z;
            y = y + 1;
        }
    }
    x
}

fn montmul(abar: u64, bbar: u64, m: u64, mprime: u64) -> u64 {
    let mut thi: u64 = 0;
    let mut tlo: u64 = 0;
    let mut tm: u64 = 0;
    let mut tmmhi: u64 = 0;
    let mut tmmlo: u64 = 0;
    let mut uhi: u64 = 0;
    let mut ulo: u64 = 0;
    let mut ov: u64 = 0;

    mulul64(abar, bbar, &mut thi, &mut tlo);

    tm = tlo * mprime;

    mulul64 (tm, m, &mut tmmhi, &mut tmmlo);

    ulo = tlo + tmmlo;
    uhi = thi + tmmhi;
    if ulo < tlo {
        uhi = uhi + 1;
    }

    ov = ((uhi < thi) | ((uhi == thi) & (ulo < tlo))) as u64;

    ulo = uhi;
    uhi = 0;

    if ov > 0 || ulo >= m {
        ulo -= m;
    }

    ulo
}

fn xbinGCD(mut a: u64, mut b: u64, pu: &mut u64, pv: &mut u64) {
    let mut u: u64 = 1;
    let mut v: u64 = 0;

    let alpha: u64 = a;
    let beta: u64 = b;

    while a > 0 {
        a = a >> 1;
        if (u & 1) == 0 {
            u = u >> 1;
            v = v >> 1;
        } else {
            u = ((u ^ beta) >> 1) + (u & beta);
            v = (v >> 1) + alpha;
        }
    }
    *pu = u;
    *pv = v;
}

fn warm_caches(heat: usize) {
    let _ = benchmark_body(heat);
}

fn benchmark_body(rpt: usize) -> usize {
    let mut errors = 0;

    const in_m: u64 = 0xfae849273928f89f;
    const in_b: u64 = 0x14736defb9330573;
    const in_a: u64 = 0x0549372187237fef;
    for _ in 0..rpt {
        let (mut hr, mut p1hi, mut p1lo, mut p1, mut p, mut abar, mut bbar): (u64, u64, u64, u64, u64, u64, u64)
            = (0, 0, 0, 0, 0, 0, 0);
        let (mut phi, mut plo): (u64, u64) = (0, 0);
        let (mut rinv, mut mprime): (u64, u64) = (0, 0);

        let m = in_m;
        let b = in_b;
        let a = in_a;

        mulul64(a, b, &mut p1hi, &mut p1lo);
        p1 = modul64(p1hi, p1lo, m);
        mulul64(p1, p1, &mut p1hi, &mut p1lo);
        p1 = modul64(p1hi, p1lo, m);
        mulul64(p1, p1, &mut p1hi, &mut p1lo);
        p1 = modul64(p1hi, p1lo, m);

        hr = 0x8000000000000000;

        xbinGCD(hr, m, &mut rinv, &mut mprime);

        if (2 * hr * rinv - m * mprime) != 1 {
            errors = 1;
        }

        abar = modul64(a, 0, m);
        bbar = modul64(b, 0, m);

        p = montmul(abar, bbar, m, mprime);
        p = montmul(p, p, m, mprime);
        p = montmul(p, p, m, mprime);

        mulul64(p, rinv, &mut phi, &mut plo);
        p = modul64(phi, plo, m);
        if p != p1 {
            errors = 1;
        }
    }
    errors
}

fn initialise_benchmark() {

}

fn verity_benchmark(r: usize) -> bool {
    0 == r
}

fn main() {

}