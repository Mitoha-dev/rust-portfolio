use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    const N: usize = 200;
    const TARGET: usize = 4;

    let v = random_array(N);
    let mut ans = None;

    for (index, &value) in v.iter().enumerate() {
        if value == TARGET {
            ans = Some(index);
            break;
        }
    }

    println!("array = {:?}", v);

    match ans {
        Some(i) => println!("index is {}", i),
        None => println!("not found"),
    }
}

fn random_array(n: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = (0..n).collect();
    arr.shuffle(&mut thread_rng());
    arr
}