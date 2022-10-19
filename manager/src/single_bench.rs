use rand::prelude::*;
use std::fs;
use std::process::Command;

pub struct SingleBenchResult {
    c: usize,
    rust_rayon: usize,
    rust_mpi: usize,
    julia: usize,
}

pub fn single_bench(
    testcase_file_path: &str,
    input_size: usize,
    kernel_count: usize,
) -> SingleBenchResult {
    // Prepare testcase
    let mut testcase = vec![0u8; input_size];
    for x in &mut testcase {
        *x = random();
    }
    let correct_result = testcase.iter().map(|x| *x as usize).sum();
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
    let rust_rayon = rust_rayon_result.time_in_micros;

    // Test rust mpi wrapper adder
    let rust_mpi_wrapper_result = process_command(
        rust_mpi_wrapper_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    if rust_mpi_wrapper_result.result != correct_result {
        panic!(
            "Rust mpi wrapper result wrong: {} vs {correct_result}!",
            rust_mpi_wrapper_result.result
        );
    }
    let rust_mpi_wrapper = rust_mpi_wrapper_result.time_in_micros;

    // Test rust mpi binding adder
    let rust_mpi_binding_result = process_command(
        rust_mpi_binding_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    if rust_mpi_binding_result.result != correct_result {
        panic!(
            "Rust mpi binding result wrong: {} vs {correct_result}!",
            rust_mpi_binding_result.result
        );
    }
    let rust_mpi_binding = rust_mpi_binding_result.time_in_micros;

    // Test c adder
    let c_result = process_command(
        c_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    if c_result.result != correct_result {
        panic!("C result wrong: {} vs {correct_result}!", c_result.result);
    }
    let c = c_result.time_in_micros;

    unimplemented!()
}

struct CommandResult {
    result: usize,
    time_in_micros: usize,
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
    println!("{}", String::from_utf8_lossy(&output.stderr));
    let mut lines = stdout_str.lines();
    let result = usize::from_str_radix(&lines.next().unwrap(), 10).unwrap();
    let time_in_micros = usize::from_str_radix(&lines.next().unwrap(), 10).unwrap();
    println!("Time is {time_in_micros}");
    CommandResult {
        result,
        time_in_micros,
    }
}

fn rust_rayon_command() -> Command {
    Command::new("./target/release/rust-rayon-adder")
}

fn rust_mpi_wrapper_command(kernel_count: usize) -> Command {
    let mut cmd = Command::new("mpiexec");
    cmd.arg("-n")
        .arg(format!("{kernel_count}"))
        .arg("./target/release/rust-mpi-adder-wrapper");
    cmd
}

fn rust_mpi_binding_command(kernel_count: usize) -> Command {
    let mut cmd = Command::new("mpiexec");
    cmd.arg("-n")
        .arg(format!("{kernel_count}"))
        .arg("./target/release/rust-mpi-adder-binding");
    cmd
}

fn c_command(kernel_count: usize) -> Command {
    let mut cmd = Command::new("mpiexec");
    cmd.arg("-n")
        .arg(format!("{kernel_count}"))
        .arg("./c-adder/c-adder");
    cmd
}
