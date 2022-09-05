use proconio::*;

fn main() {
    input! {
        s: String,
    }

    match s.as_ref() {
        "Monday" => println!("5"),
        "Tuesday" => println!("4"),
        "Wednesday" => println!("3"),
        "Thursday" => println!("2"),
        "Friday" => println!("1"),
        _ => unimplemented!(),
    }
}
