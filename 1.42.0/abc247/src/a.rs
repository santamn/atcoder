use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        mut s: Bytes,
    }
    s.insert(0, b'0');
    s.pop();
    for c in s {
        print!("{}", c as char);
    }
}
