use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        w: usize, n: usize,
        bricks: [(Usize1, Usize1); n],
    }

    let mut heights = vec![0; w];
    let mut answer = vec![0; n];
    for (i, &(l, r)) in bricks.iter().enumerate() {
        let (l, r) = if l < r { (l, r + 1) } else { (r, l + 1) };
        let &&max = &heights[l..r].iter().max().unwrap();
        answer[i] = max + 1;
        for j in l..r {
            heights[j] = max + 1;
        }
    }

    for h in answer {
        println!("{}", h);
    }
}
