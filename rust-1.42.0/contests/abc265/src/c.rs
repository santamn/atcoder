use proconio::marker::Chars;
use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h],
    }

    let mut i = 0isize;
    let mut j = 0isize;
    let mut set = HashSet::new();
    loop {
        match grid[i as usize][j as usize] {
            'R' => j += 1,
            'L' => j -= 1,
            'U' => i -= 1,
            'D' => i += 1,
            _ => unreachable!(),
        }

        if i < 0 {
            println!("{} {}", 1, j + 1);
            return;
        } else if i >= h as isize {
            println!("{} {}", h, j + 1);
            return;
        } else if j < 0 {
            println!("{} {}", i + 1, 1);
            return;
        } else if j >= w as isize {
            println!("{} {}", i + 1, w);
            return;
        }

        if set.contains(&(i, j)) {
            println!("-1");
            return;
        }
        set.insert((i, j));
    }
}
