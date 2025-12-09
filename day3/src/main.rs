use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let banks: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    // Part 1
    let mut result = 0;
    for bank in banks.iter() {
        let mut first = 0;
        let mut second = 0;

        for w in bank.windows(2) {
            if w[0] > first {
                (first, second) = (w[0], w[1]);
            } else if w[0] > second {
                second = w[0];
            } else if w[1] > second {
                second = w[1];
            }
        }

        let joltage = first * 10 + second;
        result += joltage;
    }
    println!("{result}");

    // Part 2
    let mut result = 0;
    for bank in banks.iter() {
        let mut joltage_nums = [0; 12];

        for (b_index, b) in bank.iter().copied().enumerate() {
            let replace_index = joltage_nums
                .iter()
                .enumerate()
                .find(|&(i, &v)| b > v && bank.len() - b_index >= 12 - i) // make sure there are enough remaining numbers
                .map(|(i, _)| i);
            if let Some(replace_index) = replace_index {
                joltage_nums[replace_index..].fill(0);
                joltage_nums[replace_index] = b;
            }
        }

        let joltage: u64 = joltage_nums
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * 10u64.pow(i as u32))
            .sum();
        result += joltage;
    }
    println!("{result}");
}
