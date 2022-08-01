use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize, b:usize, c:usize
    }
    println!(
        "{}",
        if (a <= b && b <= c) || (c <= b && b <= a) {
            "Yes"
        } else {
            "No"
        }
    )
}
