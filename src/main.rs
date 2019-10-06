mod common;
mod matmult_int;
mod nbody;
mod st;
mod aha_mont64;
mod crc32;
mod minver;
mod cubic;
mod nettle_aes;

use std::env;
use common::{Benchmark, WARMUP_HEAT};
use std::time::Instant;

fn select_benchmark(bench_name: &str) -> Box<dyn Benchmark> {
    match bench_name {
        "matmult_int" => return Box::new(matmult_int::MatMultIntBench::new()),
        "nbody" => return Box::new(nbody::NbodyBench::new()),
        "st" => return Box::new(st::StBench::new()),
        "aha_mont64" => return Box::new(aha_mont64::AhaMont64Bench::new()),
        "crc32" => return Box::new(crc32::CRC32Bench::new()),
        "minver" => return Box::new(minver::MinverBench::new()),
        "cubic" => return Box::new(cubic::CubicBench::new()),
        "nettle_aes" => return Box::new(nettle_aes::NettleAesBenchmark::new()),
        _ => return Box::new(matmult_int::MatMultIntBench::new()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:");
        println!("$ {} benchmark_name", args[0]);
        return;
    }

    let mut bench = select_benchmark(&args[1]);

    bench.initialise_benchmark();
    bench.warm_caches(WARMUP_HEAT);

    let start = Instant::now();
    bench.benchmark();
    let end = Instant::now();

    let correct = bench.verify_benchmark();
    if !correct {
        println!("failed");
    } else {
        println!("success");
        println!("{}ms", (end - start).as_secs_f64() * 1000.0);
    }
}