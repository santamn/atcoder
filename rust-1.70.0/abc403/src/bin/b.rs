use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: Chars, // ?を含む文字列
        u: Chars, // ?を含まない文字列
    }

    for i in 0..=(t.len() - u.len()) {
        if u.iter().zip(&t[i..]).all(|(&x, &y)| x == y || y == '?') {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
