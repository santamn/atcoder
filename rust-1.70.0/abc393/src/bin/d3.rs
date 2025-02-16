use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    println!(
        "{}",
        std::iter::zip(
            s.iter().scan(0, counter).flatten(),
            s.iter()
                .rev()
                .scan(0, counter)
                .flatten()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
        )
        .fold(0, |acc, (l, r)| acc + l.min(r))
    );
}

fn counter(acc: &mut usize, &c: &char) -> Option<Option<usize>> {
    if c == '1' {
        *acc += 1;
        Some(None)
    } else {
        Some(Some(*acc))
    }
}
