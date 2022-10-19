using MPI

function main()
    MPI.Init()
    comm = MPI.COMM_WORLD
    rank = MPI.Comm_rank(comm)
    size = MPI.Comm_size(comm)
    
    if rank == 0
        # Primary node
        input_size_str = ENV["PARALLEL_BENCH_INPUT_SIZE"]
        input_size = parse(UInt64, input_size_str)
    
        testcase_path = ENV["PARALLEL_BENCH_TESTCASE_PATH"]
        testcase = open(testcase_path, "r") do file
            raw_testcase = zeros(UInt8, input_size)
            readbytes!(file, raw_testcase)
            raw_testcase
        end
    
        next_rank = 2 * rank + 1
        top = 1
    
        while true
            if next_rank >= size
                break
            end
            next_top = div((input_size + 1 + top), 2)
            if next_top <= top
                break
            end
            MPI.Isend(testcase[top:(next_top - 1)], comm, dest=next_rank)
            top = next_top
            next_rank *= 2
        end
        sum = 0
        for i = top:input_size
            sum += convert(UInt64, testcase[i])
        end

        for i = 0:size
            partial_sum = 0
            MPI.Recv!(partial_sum, comm)
            sum += partial_sum
        end
        println(sum)
    else
        testcase = Array{UInt8}(undef, 1)
        MPI.Recv!(testcase, comm)
        input_size = length(testcase)

        next_rank = 2 * rank + 1
        top = 1
    
        while true
            if next_rank >= size
                break
            end
            next_top = div((input_size + 1 + top), 2)
            if next_top <= top
                break
            end
            MPI.Isend(testcase[top:(next_top - 1)], comm, dest=next_rank)
            top = next_top
            next_rank *= 2
        end
        sum = 0
        for i = top:input_size
            sum += convert(UInt64, testcase[i])
        end
        MPI.ISend(sum, comm, dest=1)
    end
end

main()
