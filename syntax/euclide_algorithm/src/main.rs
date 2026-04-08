fn euclid(mut n: i32, mut m: i32) -> i32 {
    // n < m の場合は交換
    if n < m {
        let temp = n;
        n = m;
        m = temp;
    }

    // 互除法
    while m != 0 {
        let r = n % m;
        n = m;
        m = r;
    }

    n
}

fn main() {
    let result = euclid(1071, 1029);
    println!("GCD = {}", result);
}