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
            .map(|optional_byte| unsafe { optional_byte.unwrap_unchecked() })
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
            unsafe {
                let to_send = testcase.get_unchecked(top..((input_size + top) / 2));
                mpi_sys::MPI_Send(
                    to_send as *const [u8] as *const libc::c_void,
                    ((input_size - top) / 2) as i32,
                    mpi_sys::RSMPI_UINT8_T,
                    next_rank,
                    0,
                    mpi_sys::RSMPI_COMM_WORLD,
                );
            }
            top = (input_size + top) / 2;
            next_rank *= 2;
        }
        let mut sum: usize = unsafe { testcase.get_unchecked(top..input_size) }
            .into_iter()
            .map(|x| *x as usize)
            .sum();
        for _ in 0..(size - 1) {
            let partial_sum = unsafe {
                let mut partial_sum: usize = 0;
                let mut status: mpi_sys::MPI_Status = std::mem::zeroed();
                mpi_sys::MPI_Recv(
                    &mut partial_sum as *mut usize as *mut libc::c_void,
                    1,
                    mpi_sys::RSMPI_UINT64_T,
                    mpi_sys::RSMPI_ANY_SOURCE,
                    0,
                    mpi_sys::RSMPI_COMM_WORLD,
                    &mut status as *mut mpi_sys::MPI_Status,
                );
                partial_sum
            };
            sum += partial_sum;
        }
        let duration = now.elapsed().as_micros();
        println!("{sum}");
        println!("{duration}");
    } else {
        let (part_testcase, input_size) = unsafe {
            let mut message: mpi_sys::MPI_Message = std::mem::zeroed();
            let mut status: mpi_sys::MPI_Status = std::mem::zeroed();
            mpi_sys::MPI_Mprobe(
                mpi_sys::RSMPI_ANY_SOURCE,
                mpi_sys::RSMPI_ANY_TAG,
                mpi_sys::RSMPI_COMM_WORLD,
                &mut message as *mut mpi_sys::MPI_Message,
                &mut status as *mut mpi_sys::MPI_Status,
            );
            let mut input_size: i32 = 0;
            mpi_sys::MPI_Get_count(
                &status as *const mpi_sys::MPI_Status,
                mpi_sys::RSMPI_UINT8_T,
                &mut input_size as *mut i32,
            );
            let mut part_testcase = vec![0u8; input_size as usize];
            mpi_sys::MPI_Mrecv(
                part_testcase.as_mut_slice() as *mut [u8] as *mut libc::c_void,
                input_size,
                mpi_sys::RSMPI_UINT8_T,
                &mut message as *mut mpi_sys::MPI_Message,
                &mut status as *mut mpi_sys::MPI_Status,
            );
            (part_testcase, input_size as usize)
        };

        let mut next_rank = 2 * rank + 1;
        let mut top = 0;

        loop {
            if (next_rank as usize) >= size {
                break;
            }
            if (input_size + top) / 2 <= top {
                break;
            }
            unsafe {
                let to_send = part_testcase.get_unchecked(top..((input_size + top) / 2));
                mpi_sys::MPI_Send(
                    to_send as *const [u8] as *const libc::c_void,
                    ((input_size - top) / 2) as i32,
                    mpi_sys::RSMPI_UINT8_T,
                    next_rank,
                    0,
                    mpi_sys::RSMPI_COMM_WORLD,
                );
            }
            top = (input_size + top) / 2;
            next_rank *= 2;
        }
        let partial_sum: usize = unsafe { part_testcase.get_unchecked(top..input_size) }
            .into_iter()
            .map(|x| *x as usize)
            .sum();
        unsafe {
            mpi_sys::MPI_Send(
                &partial_sum as *const usize as *const libc::c_void,
                1,
                mpi_sys::RSMPI_UINT64_T,
                0,
                0,
                mpi_sys::RSMPI_COMM_WORLD,
            );
        }
    }
}
