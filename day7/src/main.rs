use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let start_col = grid
        .first()
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();

    let mut beam_grid = grid.clone();
    beam_grid[0][start_col] = '|';

    let mut timeline_grid: Vec<Vec<usize>> = vec![vec![0; cols]; rows];
    timeline_grid[0][start_col] = 1;

    let mut split_count = 0;
    for r in 1..rows {
        for c in 0..cols {
            let prev = beam_grid[r - 1][c];
            let cur = beam_grid[r][c];
            match (prev, cur) {
                ('|', '^') => {
                    split_count += 1;
                    beam_grid[r][c - 1] = '|';
                    beam_grid[r][c + 1] = '|';
                    timeline_grid[r][c - 1] += timeline_grid[r - 1][c];
                    timeline_grid[r][c + 1] += timeline_grid[r - 1][c];
                }
                ('|', _) => {
                    beam_grid[r][c] = '|';
                    timeline_grid[r][c] += timeline_grid[r - 1][c];
                }
                _ => (),
            }
        }
    }

    println!("{split_count}");

    let timelines: usize = timeline_grid.last().unwrap().iter().sum();
    println!("{timelines}");
}
