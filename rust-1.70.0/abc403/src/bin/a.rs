use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().step_by(2).sum::<usize>());
}
