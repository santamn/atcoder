use proconio::*;
use std::cmp::min;
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

struct MultiSet<T> {
    map: HashMap<T, usize>,
    set: BTreeSet<T>,
}

// 重複集合: 要素の重複を許し、要素が一意(= この場合は昇順)に並んだ集合
impl<T> MultiSet<T>
where
    T: Eq + Hash + Ord + Copy,
{
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            set: BTreeSet::new(),
        }
    }

    // 重複許可挿入
    fn insert(&mut self, v: T) {
        *self.map.entry(v).or_insert(0) += 1;
        // setに要素がない場合は追加
        if self.map[&v] == 1 {
            self.set.insert(v);
        }
    }

    // 要素を一つ削除し、削除した要素を返す
    fn erase(&mut self, v: T) -> Option<T> {
        if !self.map.contains_key(&v) || self.map[&v] == 0 {
            return None;
        }

        *self.map.get_mut(&v).unwrap() -= 1;
        if self.map[&v] == 0 {
            self.set.take(&v);
        }
        Some(v)
    }

    // 要素vをc個削除。要素vがc個以下の場合はvの個数を0にする
    fn delete(&mut self, v: T, c: usize) {
        if !self.map.contains_key(&v) || self.map[&v] == 0 {
            return;
        }

        *self.map.get_mut(&v).unwrap() -= min(c, self.map[&v]);
        if self.map[&v] == 0 {
            self.set.take(&v);
        }
    }

    // 最大値を取得
    fn max(&self) -> Option<T> {
        if let Some(&v) = self.set.iter().last() {
            Some(v)
        } else {
            None
        }
    }

    // 最小値を取得
    fn min(&self) -> Option<T> {
        if let Some(&v) = self.set.iter().next() {
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    input! {q: usize}

    let mut multiset = MultiSet::new();

    for _ in 0..q {
        input! {kind: usize}

        match kind {
            1 => {
                input! {x: usize}
                multiset.insert(x);
            }
            2 => {
                input! {
                    x: usize, c:usize
                }
                multiset.delete(x, c);
            }
            3 => {
                println!("{}", multiset.max().unwrap() - multiset.min().unwrap());
            }
            _ => return,
        }
    }
}
