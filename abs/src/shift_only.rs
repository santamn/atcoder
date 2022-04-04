use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let min = println!(
        "{}",
        a.into_iter()
            .map(|mut x| {
                let mut count = 0;
                while x & 1 == 0 {
                    x = x >> 1;
                    count += 1;
                }
                count
            })
            .min()
            .unwrap()
    );
}
