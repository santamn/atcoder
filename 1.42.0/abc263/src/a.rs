use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let mut v = vec![a, b, c, d, e];
    v.sort();

    if v[0] == v[1] && v[1] == v[2] && v[3] == v[4] {
        println!("Yes");
    } else if v[0] == v[1] && v[2] == v[3] && v[3] == v[4] {
        println!("Yes");
    } else {
        println!("No");
    }
}
