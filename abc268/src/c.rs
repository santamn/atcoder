use proconio::*;
fn main() {
    input! {
        n: usize,
        p: [isize; n],
    }

    // c[i]: i/Nだけ回転させた時に喜ぶ人の人数
    let mut c = vec![0; n];

    for i in 0..n {
        for j in -1..=1 {
            let index = (p[i] - i as isize + j).rem_euclid(n as isize);
            c[index as usize] += 1;
        }
    }

    println!("{}", c.iter().max().unwrap());
}
