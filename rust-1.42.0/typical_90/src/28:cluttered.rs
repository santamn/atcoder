use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        rectangles: [(usize,usize,usize,usize); n],
    }

    let mut map = vec![[0isize; 1001]; 1001];
    for (lx, ly, rx, ry) in rectangles {
        map[lx][ly] += 1;
        map[rx][ry] += 1;
        map[lx][ry] -= 1;
        map[rx][ly] -= 1;
    }

    // 横方向の累積和
    for y in 0..=1000 {
        for x in 1..=1000 {
            map[x][y] += map[x - 1][y];
        }
    }

    // 縦方向の累積和
    for x in 0..=1000 {
        for y in 1..=1000 {
            map[x][y] += map[x][y - 1];
        }
    }

    let mut answer = vec![0; n + 1];
    for x in 0..=1000 {
        for y in 0..=1000 {
            answer[map[x][y] as usize] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", answer[i]);
    }
}
