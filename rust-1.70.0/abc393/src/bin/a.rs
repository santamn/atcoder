use proconio::*;

#[fastout]
fn main() {
    input! {
        takahashi: String,
        aoki: String,
    }

    match (takahashi.as_str(), aoki.as_str()) {
        ("sick", "sick") => println!("1"),
        ("sick", "fine") => println!("2"),
        ("fine", "sick") => println!("3"),
        ("fine", "fine") => println!("4"),
        _ => unreachable!(),
    }
}
