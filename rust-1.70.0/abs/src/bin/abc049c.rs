use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, c: usize,
        s: String,
    }

    println!("{} {}", a + b + c, s);
}
