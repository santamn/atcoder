use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        (1..=n.ilog2())
            .flat_map(|a| {
                (1..)
                    .step_by(2)
                    .map(move |b| (b * b) << a)
                    .take_while(move |&x| x <= n)
            })
            .count()
    );
}
