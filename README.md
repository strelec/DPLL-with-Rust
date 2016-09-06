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

Generating SAT from Sudokus
---
The repository includes the source code to convert Sudoku solving to SAT. Three example sudokus, are given: world's hardest 9x9 sudoku, a 16x16 sudoku and 25x25 sudoku. The latter contains 15626 variables and 752571 clauses.

To generate the DIMACS files, run:
```
target/release/generate
```
and three files are going to appear in `samples/sudokus/` directory.
