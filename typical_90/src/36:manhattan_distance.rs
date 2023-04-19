use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize, q: usize,
        p: [(isize, isize); n],
        query: [Usize1; q],
    }

    // 45度回転
    let p = p
        .into_iter()
        .map(|(a, b)| (a + b, b - a))
        .collect::<Vec<_>>();

    let (min_x, min_y, max_x, max_y) = p.iter().fold(
        (
            std::isize::MAX,
            std::isize::MAX,
            std::isize::MIN,
            std::isize::MIN,
        ),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        },
    );

    for i in query {
        println!(
            "{}",
            [
                p[i].0 - min_x,
                max_x - p[i].0,
                p[i].1 - min_y,
                max_y - p[i].1
            ]
            .iter()
            .max()
            .unwrap()
        );
    }
}
