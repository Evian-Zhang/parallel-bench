use rand::prelude::*;
use std::fs;
use std::process::Command;

pub struct SingleBenchResult {
    c: usize,
    rust_rayon: usize,
    julia: usize,
}

pub fn single_bench(
    testcase_file_path: &str,
    input_size: usize,
    kernel_count: usize,
) -> SingleBenchResult {
    // Prepare testcase
    let mut testcase = vec![0usize; input_size];
    for x in &mut testcase {
        *x = random();
    }
    let correct_result = testcase.iter().map(|x| *x).sum();
    let testcase = testcase
        .into_iter()
        .flat_map(|x| x.to_ne_bytes())
        .collect::<Vec<_>>();
    fs::write(testcase_file_path, &testcase).expect("Unable to write testcase.");

    // Test rust rayon adder
    let rust_rayon_result = process_command(
        rust_rayon_command(),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    if rust_rayon_result.result != correct_result {
        panic!("Rust rayon result wrong!");
    }
    let rust_rayon = rust_rayon_result.time_in_millis;

    unimplemented!()
}

struct CommandResult {
    result: usize,
    time_in_millis: usize,
}

fn process_command(
    mut command: Command,
    testcase_file_path: &str,
    input_size: usize,
    kernel_count: usize,
) -> CommandResult {
    let output = command
        .env("PARALLEL_BENCH_KERNEL_COUNT", format!("{kernel_count}"))
        .env("PARALLEL_BENCH_INPUT_SIZE", format!("{input_size}"))
        .env("PARALLEL_BENCH_TESTCASE_PATH", testcase_file_path)
        .output()
        .unwrap();
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout_str.lines();
    let result = usize::from_str_radix(&lines.next().unwrap(), 10).unwrap();
    let time_in_millis = usize::from_str_radix(&lines.next().unwrap(), 10).unwrap();
    CommandResult {
        result,
        time_in_millis,
    }
}

fn rust_rayon_command() -> Command {
    Command::new("./target/release/rust-rayon-adder")
}
