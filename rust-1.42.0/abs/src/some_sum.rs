use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    println!(
        "{}",
        (1..=n)
            .filter(|&num| a <= digits_sum(num) && digits_sum(num) <= b)
            .sum::<u32>()
    )
}

fn digits_sum(mut num: u32) -> u32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
