use proconio::{input, marker::Usize1};
use segment_tree::{LazySegTree, Monoid, Operator};

// maxを積にとるモノイド: (Max, -∞, max)
#[derive(Clone, Copy, PartialEq)]
enum Max {
    Num(usize),
    NegInf,
}

def_monoid! {
    Max;
    one = Self::NegInf;
    mul(self, other) = {
        match (self, other) {
            (Self::Num(n), Self::Num(m)) => Self::Num(std::cmp::max(n, m)),
            (Self::NegInf, Self::NegInf) => Self::NegInf,
            (Self::Num(n), _) | (_, Self::Num(n)) => Self::Num(n),
        }
    }
}

impl From<usize> for Max {
    fn from(n: usize) -> Self {
        Self::Num(n)
    }
}

impl From<Max> for usize {
    fn from(m: Max) -> Self {
        match m {
            Max::Num(n) => n,
            Max::NegInf => 0,
        }
    }
}

// 代入を積に取り、作用としても代入をもつモノイド: (Assign, T, 代入)
#[derive(Clone, Copy, PartialEq)]
enum Assign {
    Num(usize),
    T,
}

def_monoid! {
    Assign;
    one = Self::T;
    mul(self, other) = {
        match other {
            Self::Num(m) => Self::Num(m),
            Self::T => self,
        }
    }
}

impl Operator<Max> for Assign {
    fn act(self, x: Max) -> Max {
        match self {
            Self::Num(n) => Max::Num(n),
            Self::T => x,
        }
    }
}

impl From<usize> for Assign {
    fn from(n: usize) -> Self {
        Self::Num(n)
    }
}

fn main() {
    input! {
        _: usize, n: usize,
        bricks: [(Usize1, Usize1); n],
    }

    let bricks = compress(&bricks);
    let mut answer = vec![0; n];
    let mut lst = LazySegTree::<Max, Assign>::new(vec![0.into(); bricks.len() * 2]);

    for (i, (l, r)) in bricks.into_iter().enumerate() {
        let (l, r) = if l < r { (l, r + 1) } else { (r, l + 1) };
        let max: usize = lst.query(&(l..r)).into();
        answer[i] = max + 1;
        lst.apply(&(l..r), (max + 1).into());
    }

    answer.into_iter().for_each(|h| println!("{}", h));
}

// 座標圧縮する関数
fn compress(bricks: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ruler: Vec<usize> = bricks.iter().map(|&(l, r)| vec![l, r]).flatten().collect();
    ruler.sort();
    ruler.dedup();

    bricks
        .iter()
        .map(|&(l, r)| {
            let left = ruler.binary_search(&l).unwrap();
            let right = ruler.binary_search(&r).unwrap();
            (left, right)
        })
        .collect()
}

#[allow(dead_code)]
mod segment_tree {
    use std::ops::{Mul, MulAssign, Range};

    pub trait Monoid: Sized + PartialEq + Mul<Output = Self> + MulAssign {
        fn one() -> Self;
    }

    pub trait Operator<X> {
        fn act(self, x: X) -> X;
    }

    pub struct SegTree<X> {
        leaves: usize,
        data: Vec<X>,
    }

    impl<X> SegTree<X>
    where
        X: Copy + Monoid,
    {
        pub fn new(v: Vec<X>) -> Self {
            let leaves = next_power_of_two(v.len());
            let mut data = vec![X::one(); leaves * 2 - 1];
            for (i, k) in v.into_iter().enumerate() {
                data[i] = k;
            }
            recursive_build(&mut data, 0, leaves);
            Self { leaves, data }
        }

        pub fn update(&mut self, i: usize, x: X) {
            let mut i = i + self.leaves - 1;
            self.data[i] = x;
            while i > 0 {
                i = (i - 1) / 2;
                self.data[i] = self.data[i * 2 + 1] * self.data[i * 2 + 2];
            }
        }

        pub fn query(&mut self, range: &Range<usize>) -> X {
            self._query(range, 0, &(0..self.leaves))
        }

        fn _query(&mut self, range: &Range<usize>, i: usize, section: &Range<usize>) -> X {
            match position(range, section) {
                Relation::Separate => X::one(),
                Relation::Include => self.data[i],
                _ => {
                    let mid = (section.start + section.end) / 2;
                    self._query(range, i * 2 + 1, &(section.start..mid))
                        * self._query(range, i * 2 + 2, &(mid..section.end))
                }
            }
        }
    }

    pub struct LazySegTree<X, M> {
        leaves: usize, // 葉の数
        data: Vec<X>,  // セグメント木
        lazy: Vec<M>,  // 遅延作用
    }

    impl<X, M> LazySegTree<X, M>
    where
        X: Copy + Monoid,
        M: Copy + Monoid + Operator<X>,
    {
        pub fn new(v: Vec<X>) -> Self {
            let leaves = next_power_of_two(v.len());
            let lazy = vec![M::one(); leaves * 2 - 1];
            let mut data = vec![X::one(); leaves * 2 - 1];
            for (i, k) in v.into_iter().enumerate() {
                data[i + leaves - 1] = k;
            }
            recursive_build(&mut data, 0, leaves);
            Self { leaves, data, lazy }
        }

        fn eval(&mut self, i: usize) {
            let m = self.lazy[i];
            if m == M::one() {
                return;
            }
            if i < self.leaves - 1 {
                self.lazy[i * 2 + 1] *= m;
                self.lazy[i * 2 + 2] *= m;
            }
            self.data[i] = m.act(self.data[i]);
            self.lazy[i] = M::one();
        }

        pub fn apply(&mut self, range: &Range<usize>, m: M) {
            self._apply(range, m, 0, &(0..self.leaves));
        }

        fn _apply(&mut self, range: &Range<usize>, m: M, i: usize, section: &Range<usize>) {
            self.eval(i);
            match position(range, section) {
                Relation::Separate => {
                    return;
                }
                Relation::Include => {
                    self.lazy[i] *= m;
                    self.eval(i);
                }
                _ => {
                    let mid = (section.start + section.end) / 2;
                    self._apply(range, m, i * 2 + 1, &(section.start..mid));
                    self._apply(range, m, i * 2 + 2, &(mid..section.end));
                    self.data[i] = self.data[i * 2 + 1] * self.data[i * 2 + 2];
                }
            }
        }

        pub fn query(&mut self, range: &Range<usize>) -> X {
            self._query(range, 0, &(0..self.leaves))
        }

        fn _query(&mut self, range: &Range<usize>, i: usize, section: &Range<usize>) -> X {
            self.eval(i);
            match position(range, section) {
                Relation::Separate => X::one(),
                Relation::Include => self.data[i],
                _ => {
                    let mid = (section.start + section.end) / 2;
                    self._query(range, i * 2 + 1, &(section.start..mid))
                        * self._query(range, i * 2 + 2, &(mid..section.end))
                }
            }
        }
    }

    fn next_power_of_two(mut n: usize) -> usize {
        if n != 0 {
            n |= n >> 1;
            n |= n >> 2;
            n |= n >> 4;
            n |= n >> 8;
            n |= n >> 16;
            n |= n >> 32;
            n += 1;
        }
        n
    }

    fn recursive_build<X: Clone + Monoid>(data: &mut Vec<X>, i: usize, leaves: usize) -> X {
        if i >= leaves - 1 {
            data[i].clone()
        } else {
            data[i] =
                recursive_build(data, i * 2 + 1, leaves) * recursive_build(data, i * 2 + 2, leaves);
            data[i].clone()
        }
    }

    enum Relation {
        Separate,
        Intersect,
        Include,
        Included,
    }

    // the position of A relative to B
    fn position(a: &Range<usize>, b: &Range<usize>) -> Relation {
        if a.start <= b.start && b.end <= a.end {
            Relation::Include
        } else if b.start <= a.start && a.end <= b.end {
            Relation::Included
        } else if (a.start <= b.start && b.start < a.end) || (a.start < b.end && b.end <= a.end) {
            Relation::Intersect
        } else {
            Relation::Separate
        }
    }

    #[macro_export]
    macro_rules! def_monoid {
        (
            $type:ty where [$( $params:tt )*];
            one = $one:expr;
            mul($self:ident, $y:ident) =  $code:block
            $(;)*
        ) => {
            impl<$($params)*> std::ops::Mul for $type {
                type Output = Self;
                fn mul($self, $y: Self) -> Self { $code }
            }
            impl<$($params)*> std::ops::MulAssign for $type where Self: Clone {
                fn mul_assign(&mut $self, $y: Self) {
                    *$self = (*$self).clone() * $y;
                }
            }
            impl<$($params)*> Monoid for $type {
                #[inline(always)]
                fn one() -> Self { $one }
            }
        };

        (
            $type:ty;
            one = $one:expr;
            mul($self:ident, $y:ident) = $code:block
            $(;)*
        ) => {
            def_monoid! { $type where []; one = $one; mul($self, $y) = $code; }
        };
    }
}
