import json
import os
import matplotlib.pyplot as plt

ANALYSIS_OUTPUT_DIR = './analysis_output'
ANALYSIS_OUTPUT_PDF_DIR = f'{ANALYSIS_OUTPUT_DIR}/pdf'
ANALYSIS_OUTPUT_PNG_DIR = f'{ANALYSIS_OUTPUT_DIR}/png'
os.makedirs(ANALYSIS_OUTPUT_DIR, exist_ok=True)
os.makedirs(ANALYSIS_OUTPUT_PDF_DIR, exist_ok=True)
os.makedirs(ANALYSIS_OUTPUT_PNG_DIR, exist_ok=True)

kernel_counts = [1, 2, 4, 8, 16, 32, 64]
input_size_pows = range(20, 30)
testset = ["c", "rust_rayon", "rust_mpi_wrapper", "rust_mpi_binding", "julia"]

with open('./bench_result.json', 'r') as f:
    raw_benchmarks = json.load(f)
    benchmarks = {}
    for kernel_count in kernel_counts:
        for input_size_pow in input_size_pows:
            input_size = 2 ** input_size_pow
            single_benchmark = {}
            for test in testset:
                single_benchmark[test] = 0
            hit_count = 0
            for raw_benchmark in raw_benchmarks:
                if raw_benchmark["kernel_count"] == kernel_count and raw_benchmark["input_size"] == input_size:
                    raw_single_benchmark = raw_benchmark["single_bench_result"]
                    for test in testset:
                        single_benchmark[test] += raw_single_benchmark[test]
                    hit_count += 1
                    if hit_count == 5:
                        break
            for test in testset:
                single_benchmark[test] /= hit_count
            benchmarks[(kernel_count, input_size_pow)] = single_benchmark
    
    for kernel_count in kernel_counts:
        fig = plt.figure()
        for test in testset:
            speeds = [benchmarks[(kernel_count, input_size_pow)][test] for input_size_pow in input_size_pows]
            plt.plot(input_size_pows, speeds, label=test)
        plt.yscale("log")
        plt.legend()
        plt.xlabel("Input size (in logarithm scale)")
        plt.ylabel("Average run time in mircoseconds (in logarithm scale)")
        plt.title(f"Performance benchmark with {kernel_count} kernel")
        plt.savefig(f"{ANALYSIS_OUTPUT_PDF_DIR}/kernel-count-{kernel_count}.pdf")
        plt.savefig(f"{ANALYSIS_OUTPUT_PNG_DIR}/kernel-count-{kernel_count}.png")
    
    for input_size_pow in input_size_pows:
        fig = plt.figure()
        for test in testset:
            speeds = [benchmarks[(kernel_count, input_size_pow)][test] for kernel_count in kernel_counts]
            plt.plot(kernel_counts, speeds, label=test)
        plt.yscale("log")
        plt.legend()
        plt.xlabel("Kernel count")
        plt.ylabel("Average run time in mircoseconds (in logarithm scale)")
        plt.title(f"Performance benchmark with 2^{input_size_pow} inputs")
        plt.savefig(f"{ANALYSIS_OUTPUT_PDF_DIR}/input-size-{input_size_pow}.pdf")
        plt.savefig(f"{ANALYSIS_OUTPUT_PNG_DIR}/input-size-{input_size_pow}.png")

