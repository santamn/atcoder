use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }

    a.sort();
    b.sort();

    let sum = a.iter().zip(b).fold(0, |sum, (x, y)| sum + (x - y).abs());
    println!("{}", sum)
}
