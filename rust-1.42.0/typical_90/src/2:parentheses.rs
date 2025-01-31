use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        return;
    }

    let ps: Vec<String> = (1..(1 << n))
        .map(|num| {
            let mut parentheses = String::with_capacity(n);
            for i in (0..n).rev() {
                parentheses.push(if num & (1 << i) == 0 { '(' } else { ')' });
            }
            parentheses
        })
        .filter(|parentheses| {
            let mut score = 0;
            for p in parentheses.chars() {
                score += if p == '(' { 1 } else { -1 };
                if score < 0 {
                    return false;
                }
            }
            score == 0
        })
        .collect();

    for p in ps {
        println!("{}", p)
    }
}
