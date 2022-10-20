use rand::prelude::*;
use serde::Serialize;
use std::fs;
use std::process::Command;

#[derive(Serialize)]
pub struct SingleBenchResult {
    c: Option<usize>,
    rust_rayon: Option<usize>,
    rust_mpi_binding: Option<usize>,
    rust_mpi_wrapper: Option<usize>,
    julia: Option<usize>,
}

pub fn generate_testcase(testcase_file_path: &str, input_size: usize) -> usize {
    // Prepare testcase
    let mut testcase = vec![0u8; input_size];
    for x in &mut testcase {
        *x = random();
    }
    let correct_result = testcase.iter().map(|x| *x as usize).sum();
    fs::write(testcase_file_path, &testcase).expect("Unable to write testcase.");
    correct_result
}

pub fn single_bench(
    testcase_file_path: &str,
    input_size: usize,
    kernel_count: usize,
    correct_result: usize,
) -> SingleBenchResult {
    // Test rust rayon adder
    let rust_rayon_result = process_command(
        rust_rayon_command(),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    let rust_rayon = if let Some(result) = rust_rayon_result {
        if result.result != correct_result {
            panic!("Rust rayon result wrong!");
        }
        println!("Rust rayon time is\t\t\t{}", result.time_in_micros);
        Some(result.time_in_micros)
    } else {
        None
    };

    // Test rust mpi wrapper adder
    let rust_mpi_wrapper_result = process_command(
        rust_mpi_wrapper_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    let rust_mpi_wrapper = if let Some(result) = rust_mpi_wrapper_result {
        if result.result != correct_result {
            panic!("Rust mpi wrapper result wrong!");
        }
        println!("Rust mpi wrapper time is\t\t{}", result.time_in_micros);
        Some(result.time_in_micros)
    } else {
        None
    };

    // Test rust mpi binding adder
    let rust_mpi_binding_result = process_command(
        rust_mpi_binding_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    let rust_mpi_binding = if let Some(result) = rust_mpi_binding_result {
        if result.result != correct_result {
            panic!("Rust mpi binding result wrong!");
        }
        println!("Rust mpi binding is\t\t\t{}", result.time_in_micros);
        Some(result.time_in_micros)
    } else {
        None
    };

    // Test c adder
    let c_result = process_command(
        c_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    let c = if let Some(result) = c_result {
        if result.result != correct_result {
            panic!("C result wrong!");
        }
        println!("C time is\t\t\t\t{}", result.time_in_micros);
        Some(result.time_in_micros)
    } else {
        None
    };

    // Test Julia adder
    let julia_result = process_command(
        julia_command(kernel_count),
        testcase_file_path,
        input_size,
        kernel_count,
    );
    let julia = if let Some(result) = julia_result {
        if result.result != correct_result {
            panic!("Julia result wrong!");
        }
        println!("Julia time is\t\t\t\t{}", result.time_in_micros);
        Some(result.time_in_micros)
    } else {
        None
    };

    SingleBenchResult {
        c,
        rust_rayon,
        rust_mpi_binding,
        rust_mpi_wrapper,
        julia,
    }
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
) -> Option<CommandResult> {
    for _ in 0..32 {
        if let Some(result) =
            process_command_internal(&mut command, testcase_file_path, input_size, kernel_count)
        {
            return Some(result);
        }
        println!("Something went wrong.");
    }
    None
}

fn process_command_internal(
    command: &mut Command,
    testcase_file_path: &str,
    input_size: usize,
    kernel_count: usize,
) -> Option<CommandResult> {
    let output = command
        .env("PARALLEL_BENCH_KERNEL_COUNT", format!("{kernel_count}"))
        .env("PARALLEL_BENCH_INPUT_SIZE", format!("{input_size}"))
        .env("PARALLEL_BENCH_TESTCASE_PATH", testcase_file_path)
        .output()
        .unwrap();
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout_str.lines();
    let result = usize::from_str_radix(&lines.next()?, 10).ok()?;
    let time_in_micros = usize::from_str_radix(&lines.next()?, 10).ok()?;
    Some(CommandResult {
        result,
        time_in_micros,
    })
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

fn julia_command(kernel_count: usize) -> Command {
    let mut cmd = Command::new("mpiexec");
    cmd.arg("-n")
        .arg(format!("{kernel_count}"))
        .arg("julia")
        .arg("--project")
        .arg("./julia-adder/julia-adder.jl");
    cmd
}
