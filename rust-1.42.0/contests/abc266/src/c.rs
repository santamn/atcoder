use proconio::*;

fn main() {
    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
        d: (isize, isize),
    }

    if is_both_side(&b, &d, &a, &c) && is_both_side(&a, &c, &b, &d) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_both_side(
    p1: &(isize, isize),
    p2: &(isize, isize),
    q1: &(isize, isize),
    q2: &(isize, isize),
) -> bool {
    let line = |point: &(isize, isize)| {
        (point.1 - p1.1) as f64
            - (((p2.1 - p1.1) * (point.0 - p1.0)) as f64) / ((p2.0 - p1.0) as f64)
    };

    if p1.0 == p2.0 {
        let x = p1.0;
        (q1.0 - x) * (q2.0 - x) < 0
    } else {
        line(q1) * line(q2) < 0f64
    }
}
