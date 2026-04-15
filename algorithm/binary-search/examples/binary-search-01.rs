use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    const N: usize = 20;
    const TARGET: usize = 4;

    let mut v = random_array(N);
    v.sort(); // 二分探索はソート必須

    match binary_search(&v, TARGET) {
        Some(index) => println!("found at index {}", index),
        None => println!("not found"),
    }

    println!("array = {:?}", v);
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