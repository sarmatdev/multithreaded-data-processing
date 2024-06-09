#![feature(test)]
extern crate test;

use std::thread;
use std::thread::available_parallelism;

fn process_data<T, R>(input: Vec<T>, threshold: usize, func: fn(T) -> R) -> Vec<R>
where
    T: Clone + Send + 'static,
    R: Send + 'static,
{
    if input.len() <= threshold {
        return input.into_iter().map(|n| func(n)).collect::<Vec<R>>();
    }

    let threads_num = available_parallelism().unwrap().get();
    let chunk_size = (input.len() + threads_num - 1) / threads_num;
    let mut handles = vec![];

    for chunk in input.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || chunk.into_iter().map(|n| func(n)).collect::<Vec<R>>());
        handles.push(handle);
    }

    let mut result = vec![];

    for handle in handles {
        result.extend(handle.join().unwrap());
    }

    result
}

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

fn main() {
    let input = vec![1, 2, 3, 100];
    let threshold = 2;

    let result = process_data(input, threshold, transform_data);
    println!("{:?}", result); // [0, 1, 7, 88]
}

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
