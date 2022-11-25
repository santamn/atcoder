// 区間更新が可能になったRMQ

use std::cmp::min;
use std::mem::size_of;

struct SegmentTree {
    leaf: usize,      // 葉の数
    data: Vec<isize>, // 本体
    lazy: Vec<isize>, // 遅延作用
}

impl SegmentTree {
    fn new(v: Vec<isize>) -> Self {
        if v.len() == 0 {
            return Self {
                leaf: 0,
                data: v,
                lazy: Vec::new(),
            };
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
        let lazy = vec![std::isize::MAX; leaf * 2 - 1];

        Self::_recur(&mut data, 0, leaf);
        Self { leaf, data, lazy }
    }

    fn _recur(data: &mut Vec<isize>, i: usize, leaf: usize) -> isize {
        if i >= leaf - 1 {
            data[i]
        } else {
            data[i] = min(
                Self::_recur(data, i * 2 + 1, leaf),
                Self::_recur(data, i * 2 + 2, leaf),
            );
            data[i]
        }
    }

    // 作用を伝播する
    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == std::isize::MAX {
            return;
        }
        if i < self.leaf - 1 {
            self.lazy[i * 2 + 1] = self.lazy[i];
            self.lazy[i * 2 + 2] = self.lazy[i];
        }

        self.data[i] = self.lazy[i];
        self.lazy[i] = std::isize::MAX;
    }

    // 位置iの値をxに変更する
    fn update(&mut self, a: usize, b: usize, x: isize) {
        self._update(a, b, x, 0, 0, self.leaf);
    }

    fn _update(&mut self, a: usize, b: usize, x: isize, i: usize, l: usize, r: usize) {
        self.propagate(i);
        if a <= l && r <= b {
            // 区間が完全に内側に入る
            self.lazy[i] = x;
            self.propagate(i);
        } else if a < r && l < b {
            // 一部分だけ入る
            let mid = (l + r) / 2;
            self._update(a, b, x, i, l, mid);
            self._update(a, b, x, i, mid, r);
            self.data[i] = min(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }

    // 区間[a,b)の最小値を取得する
    fn query(&mut self, a: usize, b: usize) -> isize {
        self._query(a, b, 0, 0, self.leaf)
    }

    // 第iノードが区間[l,r)を管理している時に、区間[a,b)へのクエリに対応する
    fn _query(&mut self, a: usize, b: usize, i: usize, l: usize, r: usize) -> isize {
        self.propagate(i);

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

fn main() {}
