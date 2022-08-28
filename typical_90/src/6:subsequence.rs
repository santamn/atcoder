use proconio::marker::Bytes;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        str: Bytes,
    }

    let nex = calc_next(str);

    let mut res: Vec<char> = Vec::with_capacity(k);
    let mut j: Option<usize> = None;
    for i in 0..k {
        for c in b'a'..=b'z' {
            let l = nex[if let Some(t) = j { t + 1 } else { 0 }][(c - b'a') as usize];

            // 元の文字列の残り文字数が部分列の残り文字数より多い
            if n - l >= k - i {
                j = Some(l);
                res.push(c as char);
                break;
            }
        }
    }

    for c in res {
        print!("{}", c);
    }
}

fn calc_next(str: Vec<u8>) -> Vec<Vec<usize>> {
    let n = str.len();
    let mut v = vec![vec![n; 26]; n + 1];

    for i in (0..n).rev() {
        v[i] = v[i + 1].clone();
        v[i][(str[i] - b'a') as usize] = i;
    }

    v
}
