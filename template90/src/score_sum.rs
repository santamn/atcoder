use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize
    }

    let cossum: Vec<_> = cp
        .iter()
        .scan((0, 0), |(class1, class2), &(class, point)| {
            if class == 1 {
                *class1 += point;
            } else {
                *class2 += point;
            }
            Some((*class1, *class2))
        })
        .collect();

    for _ in 0..q {
        input! {l: Usize1, r: Usize1}
        if l == 0 {
            println!("{} {}", cossum[r].0, cossum[r].1)
        } else {
            println!(
                "{} {}",
                cossum[r].0 - cossum[l - 1].0,
                cossum[r].1 - cossum[l - 1].1
            )
        }
    }
}
