const LOCLA_SCALE_FACTOR: usize = 77;
const N: usize = 100;
const ORDER: usize = 50;

fn vec_mpy1(y: &mut [i16], x: &[i16], scaler: i16) {
    for i in 0..150 {
        y[i] += (scaler * x[i]) >> 15;
    }
}

fn mac(a: &mut [i16], b: &mut [i16], mut sqr: i32, sum: &mut i32) -> i32 {
    let mut dotp = *sum;
    for i in 0..150 {
        dotp += (b[i] * a[i]) as i32;
        sqr += (b[i] * b[i]) as i32;
    }
    *sum = dotp;
    sqr
}

fn fir(array1: &[i16], coeff: &[i16], output: &mut [i32]) {
    for i in 0..(N - ORDER) {
        let mut sum = 0;
        for j in 0..ORDER {
            sum += array1[i + j] * coeff[j];
        }
        output[i] = sum as i32 >> 15;
    }
}

fn fir_no_red_ld(x: &[i16], h: &[i16], y: &mut [i32]) {
    for j in (0..100).step_by(2) {
        let mut sum0: i32 = 0;
        let mut sum1: i32 = 0;
        let mut x0 = x[j];
        for i in (0..32).step_by(2) {
            let x1 = x[j + i + 1];
            let h0 = h[i];
            sum0 += (x0 * h0) as i32;
            sum1 += (x1 * h0) as i32;
            x0 = x[j + i + 2];
            let h1 = h[i + 1];
            sum0 += (x1 * h1) as i32;
            sum1 += (x0 * h1) as i32;
        }
        y[j] = sum0 >> 15;
        y[j + 1] = sum1 >> 15;
    }
}

fn latsynth(b: &mut [i16], k: &[i16], n: usize, mut f: i32) -> i32 {
    f -= (b[n - 1] * k[n - 1]) as i32;
    for i in (0..n-2).rev() {
        f -= (b[i] * k[i]) as i32;
        b[i + 1] = b[i] + (((k[i] as i32 * (f >> 16)) >> 16) as i16);
    }
    b[0] = (f >> 16) as i16;
    f
}

fn iirl(coefs: &[i16], input: &[i16], optr: &mut i32, state: &mut [i32]) {
    let (mut x, mut t, mut n): (i32, i32, i32) = (0, 0, 0);
    x = input[0] as i32;
    for n in 0..50 {
        t = x + ((coefs[4 * n + 2] as i32 * state[2 * n + 0] + coefs[4 * n + 3] as i32 * state[2 * n + 1]) >> 15);
        x = t + ((coefs[4 * n + 0] as i32 * state[2 * n + 0] + coefs[4 * n + 1] as i32 * state[2 * n + 1]) >> 15);
        state[1] = state[0];
        state[0] = t;
    }
    *optr = x;
}

// NOTE: codebook function is no longer needed. So, I don't implement this function.

fn jpegdct(d: &mut [i16], r: &mut [i16]) {
    let mut t = [0 as i32; 12];
    let (mut i, mut j, mut k, mut m, mut n, mut p): (i16, i16, i16, i16, i16, i16) = (0, 0, 0, 0, 0, 0);
}