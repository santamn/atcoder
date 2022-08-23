use proconio::marker::Usize1;
use proconio::*;

fn main() {
    input! {
        n: usize, m: usize, mut t: isize,
        mut a: [isize; n-1],
        bonuses: [(Usize1, isize); m],
    }

    for (index, bonus) in bonuses {
        a[index] -= bonus;
    }

    let mut count = 0;
    for time in a {
        t -= time;
        if t <= 0 {
            break;
        }
        count += 1;
    }

    if count == n - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
