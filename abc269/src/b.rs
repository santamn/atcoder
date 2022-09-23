use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut row = vec![false; 10];
    for j in 0..10 {
        let mut v = false;
        for i in 0..10 {
            if s[i][j] == '#' {
                v = true;
            }
        }
        row[j] = v;
    }

    let colom: Vec<bool> = s
        .iter()
        .map(|line| line.iter().any(|&c| c == '#'))
        .collect();

    let a = colom.iter().position(|&v| v).unwrap() + 1;
    let b = colom.iter().rposition(|&v| v).unwrap() + 1;
    let c = row.iter().position(|&v| v).unwrap() + 1;
    let d = row.iter().rposition(|&v| v).unwrap() + 1;

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
