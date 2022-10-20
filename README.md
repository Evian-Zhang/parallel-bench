# Parallel Bench

本仓库粗糙地比较了C、Rust和Julia在并行方面的性能。

## 环境

本实验的环境为24核Intel Core i9 12900K，64GB内存，Ubuntu 22.04.1系统。

MPI实现使用的是MPICH 4.0-3。

C语言使用了`mpicc`编译器。

Rust版本为1.64.0。

Julia版本为1.8.2。

## 编译

### C

```shell
mpicc -lmpi c-adder.c -O3 -o c-adder
```

采用了`O3`优化

### Rust

```shell
cargo build --release --workspace
```

采用了`release`编译选项

## 任务

由manager随机生成固定大小（2的整数次幂）的数组，程序读取并求和。

## 并行算法

对于使用MPI的`c-adder`，`rust-mpi-adder-binding`，`rust-mpi-adder-wrapper`和`julia-adder`，并行算法为：

* 数据传输阶段

  每个进程将接收的数组对半分，一半传输给下一节点（准确说是`next_rank = 2 * next_rank`）。不断对半分，直到找不到下一节点为止。

  对于节点数为`n`，这一步骤的时间复杂度为`O(log(n))`
* 数据计算阶段

  将剩余的数组内元素求和。

  对于节点数为`n`，input规模为`m`，这一步骤总共需要计算`m/log(n) - 1`次加法
* 数据收集阶段

  每个进程将得到的结果反馈给0号进程，0号进程将结果求和。

  对于节点数为`n`，数据传输需要`O(n)`，最后求和需要计算`log(n)`次加法

## 语言实现

### `c-adder`

使用C语言的MPI实现。

### `rust-mpi-adder-binding`和`rust-mpi-adder-wrapper`

Rust的MPI库是[mpi](https://crates.io/crates/mpi)。这个库对MPI的接口做了一定的封装。但是因为是今年7月才release的库，还有待打磨。

`rust-mpi-adder-wrapper`直接使用了这个库作为MPI调度接口。`rust-mpi-adder-binding`则使用了更底层的[mpi-sys](https://crates.io/crates/mpi-sys)，直接调用C接口的binding。

### `rust-rayon-adder`

[Rayon](https://crates.io/crates/rayon)是Rust原生写的并行算法库。但是与MPI不同的是，这个库的调度单元是线程，虽然开销更低，但是不适合分布式环境。由于使用起来非常简单（只需要一行代码），所以也放在这里一起比较。

### `julia-adder`

使用了[MPI.jl](https://juliaparallel.org/MPI.jl/stable/)库作为MPI调度接口。
