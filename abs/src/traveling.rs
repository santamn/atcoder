use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: isize,
        mut arr: [(isize,isize,isize); n],
    }

    arr.insert(0, (0, 0, 0));
    if arr.iter().tuple_windows().all(|(prev, next)| {
        let t = next.0 - prev.0 - ((next.1 - prev.1).abs() + (next.2 - prev.2).abs());
        t >= 0 && t % 2 == 0
    }) {
        println!("Yes")
    } else {
        println!("No")
    }
}
