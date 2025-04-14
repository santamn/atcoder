use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let (_, ans) = s
        .into_iter()
        .fold((false, 0), |(logged_in, count), s| match s.as_str() {
            "login" => (true, count),
            "logout" => (false, count),
            "private" if !logged_in => (logged_in, count + 1),
            _ => (logged_in, count),
        });

    println!("{}", ans);
}
