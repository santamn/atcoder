use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
    }

    let r = 400 % a;
    println!("{}", if r == 0 { 400 / a as isize } else { -1 });
}
