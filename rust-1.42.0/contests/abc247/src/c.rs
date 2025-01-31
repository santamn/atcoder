use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for i in s(n) {
        print!("{} ", i);
    }
}

fn s(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    } else {
        let mut v = Vec::new();
        v.append(&mut s(n - 1));
        v.push(n);
        v.append(&mut s(n - 1));
        return v;
    }
}
