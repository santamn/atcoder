use proconio::*;

fn main() {
    input! {
        y: usize,
    }

    match y % 4 {
        0 => {
            println!("{}", y + 2);
        }
        1 => {
            println!("{}", y + 1);
        }
        2 => {
            println!("{}", y);
        }
        3 => {
            println!("{}", y + 3);
        }
        _ => {
            panic!();
        }
    }
}
