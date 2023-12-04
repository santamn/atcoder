use proconio::input_interactive;

fn main() {
    input_interactive! {
        a: usize, b: usize, c: usize,
        s: String,
    };

    println!("{} {}", a + b + c, s);
}
