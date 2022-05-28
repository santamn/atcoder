use proconio::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }

    let takahashi_count = x / (a + c);
    let takahashi_mod = x % (a + c);
    let aoki_count = x / (d + f);
    let aoki_mod = x % (d + f);

    let mut takahashi = takahashi_count * a * b;
    if takahashi_mod > a {
        takahashi += a * b;
    } else {
        takahashi += takahashi_mod * b;
    }

    let mut aoki = aoki_count * d * e;
    if aoki_mod > a {
        aoki += d * e;
    } else {
        aoki += aoki_mod * e;
    }

    println!(
        "{}",
        if takahashi > aoki {
            "Takahashi"
        } else if takahashi == aoki {
            "Draw"
        } else {
            "Aoki"
        }
    )
}
