# Parallel Data Processing in Rust

This Rust project demonstrates efficient data processing using parallelism. The code leverages Rust's threading capabilities to process data in parallel when the dataset exceeds a specified threshold. It includes both single-threaded and multi-threaded processing based on the input size, making it versatile for various data sizes and system capabilities.

## Features
* _Dynamic Parallel Processing_: Automatically adjusts the number of threads based on the available parallelism on the system.
* _Customizable Data Transformation_: Supports user-defined data transformation functions.
* _Threshold-Based Parallelism_: Switches between single-threaded and multi-threaded processing depending on the input size and a user-defined threshold.

| Benchmark                                  | Time per Iteration (ns)       |
|--------------------------------------------|-------------------------------|
| bench_heavy_loaded_multi_thread_processing | 254,047.01 ns/iter (+/- 11,405.34) |
| bench_multi_thread_processing              | 51,913.97 ns/iter (+/- 1,190.89)   |
| bench_single_thread_processing             | 75.30 ns/iter (+/- 0.70)           |

### Usage
To run the main program:
```
cargo run
```

To run the tests:
```
cargo test
```

To run the benchmarks:
```
cargo bench
```

### License
_This project is licensed under the MIT License._
