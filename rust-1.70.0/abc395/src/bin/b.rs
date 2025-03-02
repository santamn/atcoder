use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    for i in 0..n {
        for j in 0..n {
            let layer = [i, j, n - 1 - i, n - 1 - j].into_iter().min().unwrap();
            if layer % 2 == 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
