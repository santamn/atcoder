use proconio::*;
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        time: f64,
        l: f64, x: f64, y: f64,
        q: usize,
    }

    for _ in 0..q {
        input! {e: f64}

        let s = (x.powf(2.0) + (y + (l / 2.0) * (2.0 * PI * e / time).sin()).powf(2.0)).sqrt();
        let t = (l / 2.0) * (1.0 - (2.0 * PI * e / time).cos());

        println!("{}", 360.0 * (t / s).atan() / (2.0 * PI));
    }
}
