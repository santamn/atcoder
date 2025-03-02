use proconio::{fastout, input_interactive, marker::Usize1};

#[fastout]
fn main() {
    input_interactive! {
        n: usize, q: usize,
    }

    // 鳩iがいる巣はもともと何か
    let mut origin_pos = (0..n).collect::<Vec<_>>();
    // もともとの巣iはいま何か
    let mut name = (0..n).collect::<Vec<_>>();
    // いまの巣iはもともと何か
    let mut inv_name = (0..n).collect::<Vec<_>>();

    for _ in 0..q {
        input_interactive! {
            t: usize,
        }
        match t {
            1 => {
                input_interactive! {
                    a: Usize1, b: Usize1,
                }
                // 鳩aを巣bへ移動
                origin_pos[a] = inv_name[b];
            }
            2 => {
                input_interactive! {
                    a: Usize1, b: Usize1,
                }
                // 巣aと巣bの交換
                (name[inv_name[a]], name[inv_name[b]]) = (name[inv_name[b]], name[inv_name[a]]);
                (inv_name[a], inv_name[b]) = (inv_name[b], inv_name[a]);
            }
            _ => {
                input_interactive! {
                    x: Usize1,
                }
                // 鳩xがいる巣の番号を報告
                println!("{}", name[origin_pos[x]] + 1);
            }
        }
    }
}
