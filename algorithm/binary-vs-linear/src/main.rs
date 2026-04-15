use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    const N: usize = 1_000_000;
    const TARGET: usize = 999_999;

    let mut v = random_array(N);
    v.sort();

    // 線形探索
    let start = Instant::now();
    let linear_result = linear_search(&v, TARGET);
    let linear_time = start.elapsed();

    // 二分探索
    let start = Instant::now();
    let binary_result = binary_search(&v, TARGET);
    let binary_time = start.elapsed();

    println!("linear search: {:?}, time = {:?}", linear_result, linear_time);
    println!("binary search: {:?}, time = {:?}", binary_result, binary_time);
}

fn linear_search(arr: &[usize], target: usize) -> Option<usize> {
    for (i, &v) in arr.iter().enumerate() {
        if v == target {
            return Some(i);
        }
    }
    None
}

fn binary_search(arr: &[usize], target: usize) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn random_array(n: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = (0..n).collect();
    arr.shuffle(&mut thread_rng());
    arr
}