use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let problem_count = rows.first().map_or(0, |r| r.len());

    // Part 1
    let mut result: u64 = 0;
    for i in 0..problem_count {
        let operator = rows.last().unwrap()[i];
        let number_rows = &rows[0..(rows.len() - 1)];

        let problem_result: u64 = match operator {
            "*" => number_rows
                .iter()
                .map(|r| r[i].parse::<u64>().unwrap())
                .product(),
            "+" => number_rows
                .iter()
                .map(|r| r[i].parse::<u64>().unwrap())
                .sum(),
            _ => panic!("unexpected operator {operator}"),
        };

        result += problem_result;
    }

    println!("{result}");

    // Part 2
    let rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    let mut result = 0;
    let mut problem_result = 0;
    let mut current_op = '*';
    for c in 0..cols {
        let op_row_char = rows.last().and_then(|r| r.get(c)).unwrap_or(&' ');
        if *op_row_char != ' ' {
            result += problem_result;
            problem_result = number_from_column(&rows, c).unwrap();
            current_op = *op_row_char;
        } else {
            match (current_op, number_from_column(&rows, c)) {
                ('*', Some(num)) => {
                    problem_result *= num;
                }
                ('+', Some(num)) => {
                    problem_result += num;
                }
                _ => (),
            }
        }
    }
    result += problem_result;

    println!("{result}");
}

fn number_from_column(rows: &[Vec<char>], column: usize) -> Option<u64> {
    let mut result: Option<u64> = None;
    for row in rows {
        let num = row.get(column).unwrap_or(&' ').to_digit(10);
        match (num, result) {
            (Some(num), Some(res)) => {
                result = Some(res * 10 + num as u64);
            }
            (Some(num), None) => {
                result = Some(num as u64);
            }
            _ => (),
        };
    }

    result
}
