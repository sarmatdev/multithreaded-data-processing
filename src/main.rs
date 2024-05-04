use std::thread;

fn process_data(input: Vec<u64>, k: u64, threshold: usize) -> Vec<u64> {
    if input.len() <= threshold {
        return input
            .into_iter()
            .map(|n| transform_data(n, k))
            .collect::<Vec<u64>>();
    }

    let mut handles = vec![];
    for chunk in input.chunks(threshold) {
        let chunk = chunk.to_vec();
        let handler = thread::spawn(move || {
            chunk
                .into_iter()
                .map(|n| transform_data(n, k))
                .collect::<Vec<u64>>()
        });
        handles.push(handler);
    }

    let mut result = vec![];

    for handle in handles {
        result.extend(handle.join().unwrap());
    }

    result
}

fn transform_data(n: u64, k: u64) -> u64 {
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
    let numbers = vec![1, 2, 3, 100];
    let threshold = 1;
    let k = 8;

    let result = process_data(numbers, k, threshold);
    println!("{:?}", result); // [0, 1, 7, 88]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_data() {
        assert_eq!(transform_data(1, 8), 0);
        assert_eq!(transform_data(2, 8), 1);
        assert_eq!(transform_data(3, 8), 7);
        assert_eq!(transform_data(100, 8), 88);
    }

    #[test]
    fn test_process_data_single_thread() {
        let numbers = vec![1, 2, 3, 100];
        let threshold = 5;
        let k = 8;
        let result = process_data(numbers.clone(), k, threshold);
        let expected_result = vec![0, 1, 7, 88];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_process_data_multi_thread() {
        let numbers = vec![1, 2, 3, 100];
        let threshold = 2;
        let k = 8;
        let result = process_data(numbers.clone(), k, threshold);
        let expected_result = vec![0, 1, 7, 88];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_process_data_multi_thread_with_larger_input() {
        let numbers = vec![1, 2, 3, 100, 267, 423];
        let threshold = 2;
        let k = 10;
        let result = process_data(numbers.clone(), k, threshold);
        let expected_result = vec![0, 1, 7, 22, 340, 3220];
        assert_eq!(result, expected_result);
    }
}
