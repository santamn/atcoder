use proconio::{marker::Chars, *};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    input! {
        s: Chars,
    }

    let mut uppercase = false;
    if let Some(_) = s.iter().find(|&&c| c.is_uppercase()) {
        uppercase = true;
    }

    let mut lowercase = false;
    if let Some(_) = s.iter().find(|&&c| c.is_lowercase()) {
        lowercase = true;
    }

    let set: HashSet<char> = HashSet::from_iter(s.iter().cloned());

    if lowercase && uppercase && set.len() == s.len() {
        println!("Yes")
    } else {
        println!("No")
    }
}
