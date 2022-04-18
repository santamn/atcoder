use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        names: [(String, String); n]
    }

    for (i, name) in names.iter().enumerate() {
        if invalid_for_nickname(&names, &name.0, i) && invalid_for_nickname(&names, &name.1, i) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn invalid_for_nickname(names: &Vec<(String, String)>, a: &String, i: usize) -> bool {
    for (j, (s, t)) in names.iter().enumerate() {
        if i != j {
            if *a == *s || *a == *t {
                return true;
            }
        }
    }
    false
}
