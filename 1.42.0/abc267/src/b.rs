use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        s: Chars,
    }

    if s[0] != '0' {
        println!("No");
        return;
    }

    let mut is_standing = vec![false; 7];
    is_standing[0] = s[6] == '1';
    is_standing[1] = s[3] == '1';
    is_standing[2] = s[1] == '1' || s[7] == '1';
    is_standing[3] = s[4] == '1';
    is_standing[4] = s[2] == '1' || s[8] == '1';
    is_standing[5] = s[5] == '1';
    is_standing[6] = s[9] == '1';
    dbg!(&is_standing);

    if let Some(mut i) = is_standing[1..6].iter().position(|&standing| !standing) {
        i += 1;
        if is_standing[..i].iter().any(|&standing| standing)
            && is_standing[i..].iter().any(|&standing| standing)
        {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
