use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, k:usize,
        a: [usize; n],
    }

    let mut right = 0usize;
    let mut left = 0usize;
    let mut answer = 0;
    let mut count = 0;
    let mut map = HashMap::<usize, usize>::new();

    while right < n {
        let n = map.entry(a[right]).or_insert(0);
        if *n > 0 || count < k {
            if *n == 0 {
                count += 1;
            }
            *n += 1;
            right += 1;
        } else {
            *map.get_mut(&a[left]).unwrap() -= 1;
            if map[&a[left]] == 0 {
                count -= 1;
            }
            left += 1;
        }

        answer = std::cmp::max(answer, right - left);
    }

    println!("{}", answer);
}
