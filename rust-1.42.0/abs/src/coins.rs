use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut count = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}
