use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if a.windows(2).all(|w| w[0] < w[1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
