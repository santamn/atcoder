use proconio::input;
use std::collections::VecDeque;

// 典型的なナップザック問題について
// [問題]
// 各品目iの重さをw_i, 価値をv_iとするとする。
// ナップザックに入れることのできる重さの合計の上限をWとしたとき、品目の価値の合計値の最大値を求めよ。
// [解法]
// 次のようなDPを考える
// dp[i番目までの品目を見る][重さの総和がj以下] = この場合の価値の総和の最大値
// これは次のような漸化式で表せる
// dp[i][j] = if j < w[i] { dp[i-1][j] } else { max(dp[i-1][j], dp[i-1][j-w_i] + v_i) }
// ただし、dp[0][j] = 0, dp[i][0] = 0

// 今回の問題について
// [問題]
// 各料理iを作る際にスパイスをL_i[mg]以上R_i[mg]以下だけ消費し(1mg単位で調整可能)、料理iは価値V_iを持つ。
// N種類の料理のうち、スパイスの消費量がW[mg]ちょうどで作れる料理の価値の合計の最大値を求めよ。(実現不可能な場合は-1)
// [解法]
// 次のようなDPを考える
// dp[i番目までの料理を見る][スパイスの消費量の総和がj] = この場合の価値の総和の最大値
// この時、漸化式は以下のようになる
// dp[i][j] = if j < L_i { dp[i-1][j] } else { max(dp[i-1][j], dp[i-1][j-L_i],...,dp[i-1][j-R_i]) + V_i}
// この時、max(dp[i-1][j], dp[i-1][j-L_i],...,dp[i-1][j-R_i]) にO(W)がかかってしまうため、全体としての計算量がO(NW^2)となってしまい、間に合わない
// そこでスライド最大値を利用して、max(dp[i-1][j], ~~dp[i-1][j-L_i],...,dp[i-1][j-R_i])をO(1)で求めることを考える~~
// スライド最大値を利用することでdpの更新が均し計算量でO(NW)ですむ(スライドが全体で一回しか起こらないので、iを固定した時の慣らし計算量がO(W))

fn main() {
    input! {
        w: usize, n: usize,
        dishes: [(usize, usize, isize); n],
    }

    let mut dp = vec![vec![std::isize::MIN / 2; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (l, r, v) = dishes[i];
        let max_index = slide_max_index(&dp[i], r - l + 1);
        for j in 0..=w {
            if j < l {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = std::cmp::max(dp[i][j], dp[i][max_index[j - l]] + v);
            }
        }
    }

    println!("{}", if dp[n][w] < 0 { -1 } else { dp[n][w] });
}

// 区間[i - K + 1, i]における v の最大値を与えるインデックスの列を返す
fn slide_max_index<T: Copy + PartialOrd>(v: &Vec<T>, k: usize) -> Vec<usize> {
    let mut max_index = vec![0; v.len()];
    let mut deq = VecDeque::new();

    for i in 0..v.len() {
        while !deq.is_empty() && v[*deq.back().unwrap()] <= v[i] {
            deq.pop_back();
        }
        deq.push_back(i);
        if i >= k && *deq.front().unwrap() == i - k {
            deq.pop_front();
        }
        max_index[i] = *deq.front().unwrap();
    }
    max_index
}

#[allow(unused_macros)]
macro_rules! debug {
    ($(a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!(a), "={:?} "),*, "|"), $(&a),*);
    };
}
