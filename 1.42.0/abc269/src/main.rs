use proconio::{input, source::line::LineSource};
use std::io::{BufRead, BufReader};

fn main() {
    let mut source = LineSource::new(BufReader::new(std::io::stdin()));
    input! {
        from &mut source,
        n: usize,
    }

    let mut a = 1;
    let mut b = n + 1;
    while b - a > 1 {
        let mid = (b + a) / 2;
        match ask(&mut source, a, mid - 1, 1, n) {
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
        match ask(&mut source, 1, n, c, mid - 1) {
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

fn ask<R: BufRead>(
    source: &mut LineSource<R>,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
) -> Result<usize, ()> {
    println!("? {} {} {} {}", a, b, c, d);

    input! {
        from source,
        t: isize,
    }
    if t < 0 {
        Err(())
    } else {
        Ok(t as usize)
    }
}
