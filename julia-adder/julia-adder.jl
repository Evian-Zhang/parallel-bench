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

        start = time_ns()

        next_nodes_count = 0
        while true
            if next_rank >= size
                break
            end
            next_top = div((input_size + 1 + top), 2)
            if next_top <= top
                break
            end
            MPI.Send(testcase[top:(next_top - 1)], comm, dest=next_rank)
            top = next_top
            next_rank *= 2
            next_nodes_count += 1
        end
        sum = 0
        for i = top:input_size
            sum += convert(UInt64, testcase[i])
        end

        for i = 1:next_nodes_count
            partial_sum = MPI.Recv(UInt64, comm)
            sum += partial_sum
        end

        duration = time_ns() - start
        println(sum)
        println(round(UInt64, duration * 1e-3))
    else
        status = MPI.Probe(MPI.ANY_SOURCE, MPI.ANY_TAG, comm)
        input_size = MPI.Get_count(status, UInt8)
        testcase = zeros(UInt8, input_size)
        MPI.Recv!(testcase, comm)
        source_rank = status.source

        next_rank = 2 * rank + 1
        top = 1
        next_nodes_count = 0

        while true
            if next_rank >= size
                break
            end
            next_top = div((input_size + 1 + top), 2)
            if next_top <= top
                break
            end
            MPI.Send(testcase[top:(next_top - 1)], comm, dest=next_rank)
            top = next_top
            next_rank *= 2
            next_nodes_count += 1
        end
        sum = 0
        for i = top:input_size
            sum += convert(UInt64, testcase[i])
        end

        for i = 1:next_nodes_count
            partial_sum = MPI.Recv(UInt64, comm)
            sum += partial_sum
        end

        MPI.Send(sum, comm, dest=source_rank)
    end
end

main()
