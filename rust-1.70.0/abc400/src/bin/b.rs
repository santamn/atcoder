use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u32,
    }

    let mut x = 0;
    for i in 0..=m {
        x += n.pow(i);
        if x > 1_000_000_000 {
            println!("inf");
            return;
        }
    }

    println!("{}", x);
}
