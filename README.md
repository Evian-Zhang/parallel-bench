# Parallel Bench

本仓库粗糙地比较了C、Rust和Julia在并行方面的性能。

## 环境

本实验的环境为24核Intel Core i9 12900K，64GB内存，Ubuntu 22.04.1系统。

MPI实现使用的是MPICH 4.0-3。

C语言使用了`mpicc`编译器。

Rust版本为
