fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let mut ans = 0;

    for i in v{
        if i == 4 {
            ans = i;
        }
    }
    println!("index is {}", ans);
}
