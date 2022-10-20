#include <mpi.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <time.h>

int main(int argc, char **argv) {
    int rank;
    int size;
    MPI_Init(&argc, &argv);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &size);

    if (rank == 0) {
        // Primary node
        char *input_size_str = getenv("PARALLEL_BENCH_INPUT_SIZE");
        if (input_size_str == NULL) { exit(-1); }
        int input_size = atoi(input_size_str);

        char *testcase_path = getenv("PARALLEL_BENCH_TESTCASE_PATH");
        if (testcase_path == NULL) { exit(-1); }
        int testcase_fd = open(testcase_path, O_RDONLY);
        if (testcase_fd == -1) { exit(-1); }
        unsigned char *testcase = (unsigned char *)malloc(input_size);
        read(testcase_fd, testcase, input_size);
        close(testcase_fd);

        int next_rank = 2 * rank + 1;
        int top = 0;

        struct timespec start;
        clock_gettime(CLOCK_MONOTONIC, &start);

        while (1) {
            if (next_rank >= size) { break; }
            if ((input_size + top) / 2 <= top) { break; }
            MPI_Send(&testcase[top], (input_size - top) / 2, MPI_UNSIGNED_CHAR, next_rank, 0, MPI_COMM_WORLD);
            top = (input_size + top) / 2;
            next_rank *= 2;
        }
        unsigned long long sum = 0;
        for (int i = top; i < input_size; i++) {
            sum += (unsigned long long)testcase[i];
        }

        for (int i = 0; i < size - 1; i++) {
            unsigned long long partial_sum;
            MPI_Status status;
            MPI_Recv(&partial_sum, 1, MPI_UNSIGNED_LONG_LONG, MPI_ANY_SOURCE, 0, MPI_COMM_WORLD, &status);
            sum += partial_sum;
        }

        struct timespec end;
        clock_gettime(CLOCK_MONOTONIC, &end);
        long seconds = (end.tv_sec - start.tv_sec);
        long micros = (((seconds * 1000000000) + end.tv_nsec) - (start.tv_nsec)) / 1000;

        printf("%llu\n", sum);
        printf("%ld\n", micros);
        free(testcase);
    } else {
        MPI_Message message;
        MPI_Status status;
        MPI_Mprobe(MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD, &message, &status);
        int input_size;
        MPI_Get_count(&status, MPI_UNSIGNED_CHAR, &input_size);
        unsigned char *part_testcase = (unsigned char *)malloc(input_size);
        MPI_Mrecv(part_testcase, input_size, MPI_UNSIGNED_CHAR, &message, &status);

        int next_rank = 2 * rank + 1;
        int top = 0;

        while (1) {
            if (next_rank >= size) { break; }
            if ((input_size + top) / 2 <= top) { break; }
            MPI_Send(&part_testcase[top], (input_size - top) / 2, MPI_UNSIGNED_CHAR, next_rank, 0, MPI_COMM_WORLD);
            top = (input_size + top) / 2;
            next_rank *= 2;
        }
        unsigned long long sum = 0;
        for (int i = top; i < input_size; i++) {
            sum += (unsigned long long)part_testcase[i];
        }
        MPI_Send(&sum, 1, MPI_UNSIGNED_LONG_LONG, 0, 0, MPI_COMM_WORLD);

        free(part_testcase);
    }

    MPI_Finalize();

    return 0;
}
