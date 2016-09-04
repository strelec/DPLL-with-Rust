DPLL solver in rust
===

Compiling
---
First, you have to install `Rust` and `Cargo` (the package manager) on your system. You then run:
 
```
cargo build --release
```

Running
---
Once you have a binary, you can either pass your file as a parameter or pipe it into the standard input. Both examples below:
```
target/release/solve samples/easy/19x19queens.txt
target/release/solve < samples/easy/19x19queens.txt
```

Benchmarking
---
There are a number of samples in the `samples` directory. To run them all, from easiest to hardest, run the benchmark script:
```
sh benchmark.sh
```
It will display the CPU usage in seconds and maximum RAM usage in KiB.
