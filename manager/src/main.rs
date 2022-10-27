mod single_bench;

use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt)]
enum CmdlineOption {
    Full,
    Single(SingleBenchOption),
}

#[derive(StructOpt)]
struct SingleBenchOption {
    #[structopt(long)]
    kernel_count: usize,
    #[structopt(long)]
    input_size: usize,
}

#[derive(Serialize, Debug)]
struct BenchResult {
    kernel_count: usize,
    input_size: usize,
    round: usize,
    single_bench_result: single_bench::SingleBenchResult,
}

fn main() {
    let cmdline_option = CmdlineOption::from_args();
    match cmdline_option {
        CmdlineOption::Full => {
            let mut bench_results = vec![];
            for power in 20u32..30u32 {
                let input_size = 2usize.pow(power);
                let correct_result = single_bench::generate_testcase("./testcase.bin", input_size);
                for kernel_count in [1, 2, 4, 6, 8, 10, 12, 14, 16] {
                    for round in 0..16 {
                        println!("Input size is 2^{power} = {input_size}; Kernel count is {kernel_count}");
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
        CmdlineOption::Single(SingleBenchOption {
            kernel_count,
            input_size,
        }) => {
            let correct_result = single_bench::generate_testcase("./testcase.bin", input_size);
            let single_bench_result = single_bench::single_bench(
                "./testcase.bin",
                input_size,
                kernel_count,
                correct_result,
            );
            println!("{single_bench_result:?}")
        }
    }
}
