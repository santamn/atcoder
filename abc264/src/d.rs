use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        mut s: Chars,
    }

    let a = s.iter().position(|&x| x == 'a').unwrap();
    s.remove(a);
    let t = s.iter().position(|&x| x == 't').unwrap();
    s.remove(t);
    let c = s.iter().position(|&x| x == 'c').unwrap();
    s.remove(c);
    let o = s.iter().position(|&x| x == 'o').unwrap();
    s.remove(o);
    let d = s.iter().position(|&x| x == 'd').unwrap();
    s.remove(d);
    let e = s.iter().position(|&x| x == 'e').unwrap();

    println!("{}", a + t + c + o + d + e);
}
