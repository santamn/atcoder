use proconio::{input, source::line::LineSource};
use std::io::BufReader;

fn main() {
    input! {
        from &mut LineSource::new(BufReader::new(std::io::stdin())),
        n: usize,
    }

    let mut a = 1;
    let mut b = n + 1;
    while b - a > 1 {
        let mid = (b + a) / 2;
        match ask(a, mid - 1, 1, n) {
            Ok(t) => {
                if t == mid - a {
                    a = mid;
                } else {
                    b = mid;
                }
            }
            Err(_) => {
                return;
            }
        }
    }

    let mut c = 1;
    let mut d = n + 1;
    while d - c > 1 {
        let mid = (d + c) / 2;
        match ask(1, n, c, mid - 1) {
            Ok(t) => {
                if t == mid - c {
                    c = mid;
                } else {
                    d = mid;
                }
            }
            Err(_) => {
                return;
            }
        }
    }

    println!("! {} {}", a, c);
}

fn ask(a: usize, b: usize, c: usize, d: usize) -> Result<usize, ()> {
    println!("? {} {} {} {}", a, b, c, d);

    input! {
        from &mut LineSource::new(BufReader::new(std::io::stdin())),
        t: isize,
    }
    if t < 0 {
        Err(())
    } else {
        Ok(t as usize)
    }
}
