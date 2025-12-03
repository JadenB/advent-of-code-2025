use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();

    // Part 1
    let mut result = 0;
    for range in input.trim().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for num in start..=end {
            if num.ilog10() % 2 == 0 {
                // Odd number of digits
                continue;
            }

            let pow = 10u64.pow(num.ilog10() / 2 + 1);
            let left = num / pow;
            let right = num % pow;

            if left == right {
                result += num;
            }
        }
    }
    println!("{result}");

    // Part 2
    let mut result = 0;
    for range in input.trim().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for num in start..=end {
            if repeats_at_least_twice(num) {
                result += num;
            }
        }
    }
    println!("{result}");
}

fn repeats_at_least_twice(num: u64) -> bool {
    if num < 10 {
        return false;
    }

    for pow in 1..=(num.ilog10() / 2 + 1) {
        let pow = 10u64.pow(pow);
        let maybe_repeating = num % pow;

        if maybe_repeating < pow / 10 {
            // skip leading zeroes
            continue;
        }

        let mut num = num;
        loop {
            if num == 0 {
                return true;
            } else if num % pow != maybe_repeating {
                break;
            }

            num /= pow;
        }
    }

    false
}
