use std::fs;

struct Grid {
    data: Vec<Vec<char>>,
    rows: i64,
    columns: i64,
}

impl Grid {
    fn from_string(string: &str) -> Self {
        let data: Vec<Vec<char>> = string.lines().map(|l| l.chars().collect()).collect();
        let rows = data.len() as i64;
        let columns = data.iter().map(|r| r.len()).min().unwrap_or(0) as i64;
        Self {
            data,
            rows,
            columns,
        }
    }

    fn at_or(&self, row: i64, column: i64, or: char) -> char {
        if (0..self.rows).contains(&row) && (0..self.columns).contains(&column) {
            self.data[row as usize][column as usize]
        } else {
            or
        }
    }

    fn set(&mut self, row: i64, column: i64, val: char) {
        self.data[row as usize][column as usize] = val;
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let mut grid = Grid::from_string(&input);

    // Part 1
    let mut result = 0;
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            let adjacent = [
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
                (row, col - 1),
                (row, col + 1),
                (row + 1, col - 1),
                (row + 1, col),
                (row + 1, col + 1),
            ]
            .iter()
            .filter(|(r, c)| grid.at_or(*r, *c, '.') == '@')
            .count();

            if grid.at_or(row, col, '.') == '@' && adjacent < 4 {
                result += 1;
            }
        }
    }
    println!("{result}");

    // Part 2
    let mut result = 0;
    loop {
        let mut removed = 0;
        for row in 0..grid.rows {
            for col in 0..grid.columns {
                let adjacent = [
                    (row - 1, col - 1),
                    (row - 1, col),
                    (row - 1, col + 1),
                    (row, col - 1),
                    (row, col + 1),
                    (row + 1, col - 1),
                    (row + 1, col),
                    (row + 1, col + 1),
                ]
                .iter()
                .filter(|(r, c)| grid.at_or(*r, *c, '.') == '@')
                .count();

                if grid.at_or(row, col, '.') == '@' && adjacent < 4 {
                    result += 1;
                    grid.set(row, col, '.');
                    removed += 1;
                }
            }
        }

        if removed == 0 {
            break;
        }
    }

    println!("{result}");
}
