use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        chars: Chars,
    }

    println!(
        "{}",
        chars.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
    )
}
