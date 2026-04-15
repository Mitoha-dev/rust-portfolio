use rand::seq::SliceRandom;
use rand::thread_rng;


fn main() {

    const N: usize = 20;
    const TARGET = 4;

    let v = random_array(N);
    let mut ans = 0;

    for i in &v{
        if v[*i] == TARGET {
            ans = *i;
        }
    }
    println!("index is {}", ans+1);
    println!("{:?}", v);
}



fn random_array(n: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = (0..n).collect();
    arr.shuffle(&mut thread_rng());
    arr
}