use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let a = s
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '1' { Some(i) } else { None })
        .enumerate()
        .map(|(i, place)| (place - i) as i64)
        .collect::<Vec<_>>();

    let median = a[a.len() / 2];
    println!("{}", a.iter().fold(0, |acc, &x| acc + (median - x).abs()));
}
