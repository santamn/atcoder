use proconio::*;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        mut classes: [isize; n],
        q: usize,
        students: [isize; q],
    }

    classes.sort();
    for student in &students {
        match classes.binary_search(student).unwrap_or_else(|x| x) {
            0 => {
                println!("{}", classes[0] - student);
            }
            i if i == n => {
                println!("{}", (student - classes[i - 1]));
            }
            i => {
                println!(
                    "{}",
                    min(
                        (classes[i - 1] - student).abs(),
                        (classes[i] - student).abs()
                    )
                );
            }
        }
    }
}
