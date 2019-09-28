
static SEED: usize = 0;

fn rand_beebs() -> i32 {
  seed = (seed * 1103515245 + 12345) & ((1 << 31) - 1);
  (seed >> 16) as i32
}

fn srand_beebs(new_seed: usize) {
    SEED = new_seed;
}