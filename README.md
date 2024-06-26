# Parallel Data Processing in Rust

This Rust project demonstrates efficient data processing using parallelism. The code leverages Rust's threading capabilities to process data in parallel when the dataset exceeds a specified threshold. It includes both single-threaded and multi-threaded processing based on the input size, making it versatile for various data sizes and system capabilities.

## Features
Dynamic Parallel Processing: Automatically adjusts the number of threads based on the available parallelism on the system.
Customizable Data Transformation: Supports user-defined data transformation functions.
Threshold-Based Parallelism: Switches between single-threaded and multi-threaded processing depending on the input size and a user-defined threshold.
Unit Tests and Benchmarks: Comprehensive testing and benchmarking to ensure correctness and performance.
Usage
Functionality
The main function provided is process_data, which processes input data using a specified transformation function. If the input size exceeds the threshold, it processes the data in parallel using multiple threads.

## Example

```
fn main() {
    let input = vec![1, 2, 3, 100];
    let threshold = 2;

    let result = process_data(input, threshold, transform_data);
    println!("{:?}", result); // [0, 1, 7, 88]
}
```
### Data Transformation
The example transformation function transform_data implements a variation of the Collatz conjecture with a limit on the number of iterations:

```
fn transform_data(n: u64) -> u64 {
    let k = 8;
    let mut value = n;
    let mut iterations = 0;

    while value != 1 && iterations < k {
        if value % 2 == 0 {
            value /= 2;
        } else {
            value = value * 3 + 1;
        }
        iterations += 1;
    }

    if iterations >= k {
        value
    } else {
        iterations
    }
}
```
### Testing
Unit tests are included to verify the correctness of both the transformation and the parallel processing logic:

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_data() {
        assert_eq!(transform_data(1), 0);
        assert_eq!(transform_data(2), 1);
        assert_eq!(transform_data(3), 7);
        assert_eq!(transform_data(100), 88);
    }

    #[test]
    pub fn test_process_data_single_thread() {
        let input = vec![1, 2, 3, 100];
        let threshold = 5;
        let result = process_data(input, threshold, transform_data);
        let expected_result = vec![0, 1, 7, 88];
        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn test_process_data_multi_thread() {
        let input = vec![1, 2, 3, 100];
        let threshold = 2;
        let result = process_data(input, threshold, transform_data);
        let expected_result = vec![0, 1, 7, 88];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_process_data_multi_thread_with_larger_input() {
        let input = vec![1, 2, 3, 100, 267, 423];
        let threshold = 2;
        let result = process_data(input, threshold, transform_data);
        let expected_result = vec![0, 1, 7, 22, 340, 3220];
        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn test_process_data_multi_thread_heavy_loaded() {
        let input = (0u64..100_000).collect::<Vec<u64>>();
        let threshold = 500;
        let result = process_data(input, threshold, transform_data);
        assert_eq!(result.len(), 100_000);
    }
}
```
### Benchmarks
Benchmarks are provided to measure the performance of the single-threaded and multi-threaded processing:

```
#[cfg(test)]
mod benchmarks {
    use crate::tests::*;
    use test::Bencher;

    #[bench]
    fn bench_single_thread_processing(b: &mut Bencher) {
        b.iter(|| test_process_data_single_thread());
    }

    #[bench]
    fn bench_multi_thread_processing(b: &mut Bencher) {
        b.iter(|| test_process_data_multi_thread());
    }

    #[bench]
    fn bench_heavy_loaded_multi_thread_processing(b: &mut Bencher) {
        b.iter(|| test_process_data_multi_thread_heavy_loaded());
    }
}
```
### Requirements
Rust (latest stable version recommended)
Nightly Rust toolchain for benchmarking
Running the Code
To run the main program:

```
cargo run
```
To run the tests:
```
cargo test
```
To run the benchmarks (requires nightly Rust):
```
cargo +nightly bench
```

License
This project is licensed under the MIT License.
