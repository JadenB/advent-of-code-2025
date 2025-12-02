use std::fs;

const DIAL_SIZE: i32 = 100;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();

    let mut password = 0;
    let mut dial = 50;
    for line in input.lines() {
        let magnitude: i32 = line[1..].parse().unwrap();

        let rotation = if line.starts_with("L") {
            -magnitude
        } else {
            magnitude
        };

        dial = (dial + DIAL_SIZE + rotation) % DIAL_SIZE;
        if dial == 0 {
            password += 1;
        }
    }

    println!("{password}");
}
