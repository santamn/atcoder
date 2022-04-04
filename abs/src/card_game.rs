use proconio::input;

fn main() {
    input! {
        n: u32,
        mut cards: [isize; n],
    }

    cards.sort_by_key(|v| -v);
    let diff = cards
        .into_iter()
        .enumerate()
        .fold(0isize, |s, (i, v)| s + (1 - 2 * (i as isize & 1)) * v);
    println!("{}", diff)
}
