use proconio::{fastout, input_interactive, marker::Usize1};
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Permission {
    All,
    Individual(HashSet<usize>),
}

impl Permission {
    fn new() -> Self {
        Permission::Individual(HashSet::new())
    }

    fn allow_all(&mut self) {
        *self = Permission::All;
    }

    fn allow_individual(&mut self, problem: usize) {
        if let Permission::Individual(set) = self {
            set.insert(problem);
        }
    }

    fn is_allowed(&self, problem: usize) -> bool {
        match self {
            Permission::All => true,
            Permission::Individual(set) => set.contains(&problem),
        }
    }
}

#[fastout]
fn main() {
    input_interactive! {
        n: usize, // ユーザー数
        _: usize, // 問題数
        q: usize,
    }

    let mut users = vec![Permission::new(); n];

    for _ in 0..q {
        input_interactive! {
            t: usize,
            x: Usize1, // ユーザー番号
        }

        match t {
            1 => {
                // ユーザーxに問題yを許可
                input_interactive! {
                    y: usize, // 問題番号
                }
                users[x].allow_individual(y);
            }
            2 => {
                // ユーザーxに全問題を許可
                users[x].allow_all();
            }
            _ => {
                input_interactive! {
                    y: usize, // 問題番号
                }

                println!("{}", if users[x].is_allowed(y) { "Yes" } else { "No" })
            }
        }
    }
}
