use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut count = 0;
    for i in 0..n {
        if s[i] != 'A' {
            continue;
        }

        for j in i + 1..n {
            if s[j] != 'B' {
                continue;
            }

            for k in j + 1..n {
                if s[k] == 'C' && j - i == k - j {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
