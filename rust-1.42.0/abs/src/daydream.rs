use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|&s| s.chars().rev().collect())
        .collect();
    let s: Vec<char> = s.chars().rev().collect();
    let mut s = &s[..];
    let mut succeeded = true;

    while s.len() > 0 {
        match patterns.iter().find(|&p| s.starts_with(p)) {
            Some(p) => {
                s = &s[p.len()..];
            }
            None => {
                succeeded = false;
                break;
            }
        }
    }
    println!("{}", if succeeded { "YES" } else { "NO" })
}
