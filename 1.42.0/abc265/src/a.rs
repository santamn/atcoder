use proconio::*;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize
    }

    if n > 2 && y < 3 * x {
        println!("{}", (n / 3) * y + (n % 3) * x);
    } else {
        println!("{}", n * x);
    }
}
