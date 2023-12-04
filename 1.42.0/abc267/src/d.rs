use proconio::*;
fn main() {
    input! {
        n: usize, m: usize,
        a: [isize; n],
    }

    let mut c: isize = a[..m]
        .iter()
        .enumerate()
        .map(|(i, v)| (i as isize + 1) * v)
        .sum();
    let mut record = a[..m].to_vec();

    for i in 0..(n - m) {
        let (id, sum): (usize, isize) = (0..m)
            .map(|x| {
                (
                    x,
                    record
                        .iter()
                        .enumerate()
                        .filter(move |&(j, _)| x != j)
                        .map(|(_, v)| v),
                )
            })
            .map(|(x, iter)| (x, iter.enumerate().map(|(j, v)| (j as isize + 1) * v).sum()))
            .max_by_key(|&(_, v)| v)
            .unwrap();
        let x = sum + (m as isize) * a[i + m];

        if c < x {
            c = x;
            record.remove(id);
            record.push(a[i + m]);
        }
    }

    println!("{}", c);
}
