// 区間上の最小値 Range Minimum Query(RMQ)の実装

use std::cmp::min;
use std::mem::size_of;

struct SegmentTree {
    leaf: usize,      // 葉の数
    data: Vec<isize>, // 本体
}

impl SegmentTree {
    fn new(v: Vec<isize>) -> Self {
        if v.len() == 0 {
            return Self { leaf: 0, data: v };
        }

        let n = v.len();
        let leaf = if n & (n - 1) == 0 {
            n
        } else {
            1 << (size_of::<usize>() * 8 - n.leading_zeros() as usize)
        };

        let mut data = vec![std::isize::MAX; leaf * 2 - 1];
        for (i, &k) in v.iter().enumerate() {
            data[i + leaf - 1] = k;
        }

        recur(&mut data, 0, leaf);
        Self { leaf, data }
    }

    // 位置iの値をxに変更する
    fn update(&mut self, i: usize, x: isize) {
        let mut i = i + self.leaf - 1;
        self.data[i] = x;
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = min(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }

    // 区間[a,b)の最小値を取得する
    fn query(&self, a: usize, b: usize) -> isize {
        self._query(a, b, 0, 0, self.leaf)
    }

    // 第iノードが区間[l,r)を管理している時に、区間[a,b)へのクエリに対応する
    fn _query(&self, a: usize, b: usize, i: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            // クエリが管理区間外
            std::isize::MAX
        } else if a <= l && r <= b {
            // クエリが管理区間にすっぽり収まる
            self.data[i]
        } else {
            let m = (l + r) / 2;
            min(
                self._query(a, b, i * 2 + 1, l, m),
                self._query(a, b, i * 2 + 2, m, r),
            )
        }
    }
}

fn recur(data: &mut Vec<isize>, i: usize, leaf: usize) -> isize {
    if i >= leaf - 1 {
        data[i]
    } else {
        data[i] = min(recur(data, i * 2 + 1, leaf), recur(data, i * 2 + 2, leaf));
        data[i]
    }
}

fn main() {
    let v = vec![5, 4, 3, 8, 1, 7];
    let mut tree = SegmentTree::new(v);

    println!("{:?}", tree.leaf);
    println!("{:?}", tree.data);
    tree.update(3, 2);
    println!("updated: {:?}", tree.data);
    println!("{:?}", tree.query(1, 5));
}
