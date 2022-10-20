mod single_bench;

use serde::Serialize;

#[derive(Serialize)]
struct BenchResult {
    kernel_count: usize,
    input_size: usize,
    round: usize,
    single_bench_result: single_bench::SingleBenchResult,
}

fn main() {
    let mut bench_results = vec![];
    for power in 20u32..30u32 {
        let input_size = 2usize.pow(power);
        let correct_result = single_bench::generate_testcase("./testcase.bin", input_size);
        for kernel_count in [1, 2, 4, 8, 16, 32, 64] {
            for round in 0..16 {
                println!("2^{power} = {input_size}: {kernel_count} kernel");
                let single_bench_result = single_bench::single_bench(
                    "./testcase.bin",
                    input_size,
                    kernel_count,
                    correct_result,
                );
                bench_results.push(BenchResult {
                    kernel_count,
                    input_size,
                    round,
                    single_bench_result,
                })
            }
        }
    }
    std::fs::write(
        "./bench_result.json",
        serde_json::to_string(&bench_results).unwrap(),
    );
}
