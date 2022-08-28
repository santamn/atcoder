use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.insert(0, 0);
    a.push(l);
    let yokans: Vec<usize> = a
        .iter()
        .tuple_windows()
        .map(|(prev, next)| next - prev)
        .collect();

    let mut ok = 0;
    let mut ng = l;
    while ng - ok > 1 {
        let score = (ok + ng) / 2;
        if score_realizable(&yokans, k, score) {
            ok = score;
        } else {
            ng = score;
        }
    }

    println!("{}", ok)
}

fn score_realizable(yokans: &Vec<usize>, mut cut: usize, score: usize) -> bool {
    let mut concat = 0;

    for (idx, yokan) in yokans.iter().enumerate() {
        concat += yokan;
        if concat >= score {
            concat = 0;
            cut -= 1;
            if cut == 0 {
                return yokans[(idx + 1)..].iter().sum::<usize>() >= score;
            }
        }
    }
    false
}
