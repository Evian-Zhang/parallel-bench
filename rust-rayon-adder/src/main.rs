use itertools::Itertools;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn calculate_sum(testcase: &[usize]) -> usize {
    testcase.par_iter().sum()
}

fn main() {
    let kernel_count_str = env::var("PARALLEL_BENCH_KERNEL_COUNT").unwrap();
    let kernel_count = usize::from_str_radix(&kernel_count_str, 10).unwrap();
    rayon::ThreadPoolBuilder::new()
        .num_threads(kernel_count)
        .build_global()
        .unwrap();

    let testcase_path = env::var("PARALLEL_BENCH_TESTCASE_PATH").unwrap();
    let testcase_file = File::open(testcase_path).unwrap();
    let testcase = testcase_file
        .bytes()
        .map(|optional_byte| unsafe { optional_byte.unwrap_unchecked() })
        .tuples()
        .map(|(a1, a2, a3, a4, a5, a6, a7, a8)| {
            usize::from_ne_bytes([a1, a2, a3, a4, a5, a6, a7, a8])
        })
        .collect::<Vec<_>>();

    let now = Instant::now();

    let result = calculate_sum(testcase.as_slice());

    let duration = now.elapsed().as_millis();
    println!("{result}");
    println!("{duration}");
}
