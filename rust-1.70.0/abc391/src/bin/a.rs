use proconio::*;

#[fastout]
fn main() {
    input! {
        d: String,
    }

    if d == "N" {
        println!("S");
    } else if d == "S" {
        println!("N");
    } else if d == "E" {
        println!("W");
    } else if d == "W" {
        println!("E");
    } else if d == "NE" {
        println!("SW");
    } else if d == "NW" {
        println!("SE");
    } else if d == "SE" {
        println!("NW");
    } else {
        println!("NE");
    }
}
