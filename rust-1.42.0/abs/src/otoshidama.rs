use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }

    for i in 0..=n {
        for j in 0..=(n - i) {
            if 10000 * i + 5000 * j + 1000 * (n - i - j) == y {
                println!("{} {} {}", i, j, n - i - j);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
