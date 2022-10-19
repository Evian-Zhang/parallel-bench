use mpi::traits::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size() as usize;
    let rank = world.rank();

    if rank == 0 {
        // Primary node
        let testcase_path = env::var("PARALLEL_BENCH_TESTCASE_PATH").unwrap();
        let testcase_file = File::open(testcase_path).unwrap();
        let testcase = testcase_file
            .bytes()
            .map(|optional_byte| unsafe { optional_byte.unwrap_unchecked() } as usize)
            .collect::<Vec<_>>();
        let input_size = testcase.len();

        let mut next_rank = 2 * rank + 1;
        let mut top = 0;

        let now = Instant::now();

        loop {
            if (next_rank as usize) >= size {
                break;
            }
            if (input_size + top) / 2 <= top {
                break;
            }
            world
                .process_at_rank(next_rank)
                .send(unsafe { testcase.get_unchecked(top..((input_size + top) / 2)) });
            top = (input_size + top) / 2;
            next_rank *= 2;
        }
        let mut sum: usize = unsafe { testcase.get_unchecked(top..input_size) }
            .into_iter()
            .sum();
        for _ in 0..(size - 1) {
            let (partial_sum, _) = world.any_process().receive::<usize>();
            sum += partial_sum;
        }
        let duration = now.elapsed().as_micros();
        println!("{sum}");
        println!("{duration}");
    } else {
        let (part_testcase, _) = world.any_process().receive_vec::<usize>();
        let input_size = part_testcase.len();

        let mut next_rank = 2 * rank + 1;
        let mut top = 0;

        loop {
            if (next_rank as usize) >= size {
                break;
            }
            if (input_size + top) / 2 <= top {
                break;
            }
            world
                .process_at_rank(next_rank)
                .send(unsafe { part_testcase.get_unchecked(top..((input_size + top) / 2)) });
            top = (input_size + top) / 2;
            next_rank *= 2;
        }
        let partial_sum: usize = unsafe { part_testcase.get_unchecked(top..input_size) }
            .into_iter()
            .sum();
        world.process_at_rank(0).send(&partial_sum);
    }
}
