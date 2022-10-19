mod single_bench;

fn main() {
    single_bench::single_bench("./testcase.bin", 1024 * 1024, 4);
}
