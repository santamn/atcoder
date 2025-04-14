use proconio::{fastout, input};

const MOD: usize = 1_000_000_000;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
    }

    if k > n {
        println!("1");
        return;
    }

    let mut a = vec![1; n + 1];
    a[k] = k;
    for i in (k + 1)..=n {
        a[i] = (a[i - 1] % MOD * 2 - a[i - k - 1] % MOD + MOD) % MOD;
    }

    println!("{}", a[n]);
}
