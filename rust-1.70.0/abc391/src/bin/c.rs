use proconio::{marker::Usize1, *};

fn main() {
    input_interactive! {
        n: usize, q: usize,
    }

    let mut crowded_nests = 0;
    let mut nests = vec![1; n]; // 各巣の鳩の数
    let mut pigeons: Vec<usize> = (0..n).collect(); // 各鳩の巣の番号

    for _ in 0..q {
        input_interactive! {
            kind: usize,
        }

        if kind == 1 {
            // 鳩pを巣hに入れる
            input_interactive! {
                pigeon: Usize1, new_nest: Usize1
            }

            let old_nest = pigeons[pigeon];
            nests[old_nest] -= 1;
            nests[new_nest] += 1;
            if nests[old_nest] == 1 {
                crowded_nests -= 1;
            }
            if nests[new_nest] == 2 {
                crowded_nests += 1;
            }
            pigeons[pigeon] = new_nest;
        } else {
            println!("{}", crowded_nests);
        }
    }
}
