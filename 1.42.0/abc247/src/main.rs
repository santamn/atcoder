use proconio::*;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    for _ in 0..q {
        input! {
            n: usize
        }

        if n == 1 {
            input! {
                x: usize,
                c: usize,
            }
            que.push_front((x, c));
        } else {
            input! { mut c: usize }

            let mut sum = 0;
            while c > 0 {
                let (y, d) = que.pop_back().unwrap();
                if d > c {
                    sum += y * c;
                    que.push_back((y, d - c));
                    c = 0;
                } else {
                    sum += y * d;
                    c -= d;
                }
            }

            println!("{}", sum);
        }
    }
}
