use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let one_places = s
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '1' { Some(i) } else { None })
        .collect::<Vec<_>>();

    let median_index = one_places.len() / 2;
    let median = one_places[median_index] as i64;
    println!(
        "{}",
        one_places
            .iter()
            .enumerate()
            .map(|(i, &place)| {
                (median - place as i64 - 1).abs() - (median_index as i64 - i as i64 - 1).abs()
            })
            .sum::<i64>()
    );
}
