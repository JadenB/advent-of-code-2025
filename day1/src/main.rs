use std::fs;

const DIAL_SIZE: i32 = 100;

fn positive_modulo(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();

    // Part 1
    let mut password = 0;
    let mut dial = 50;
    for line in input.lines() {
        let magnitude: i32 = line[1..].parse().unwrap();

        let rotation = if line.starts_with("L") {
            -magnitude
        } else {
            magnitude
        };

        dial = positive_modulo(dial + rotation, DIAL_SIZE);
        if dial == 0 {
            password += 1;
        }
    }
    println!("{password}");

    // Part 2
    let mut password = 0;
    let mut dial = 50;
    for line in input.lines() {
        let magnitude: i32 = line[1..].parse().unwrap();

        let (rotation, distance_past_zero) = if line.starts_with("L") {
            (-magnitude, (DIAL_SIZE - dial) % DIAL_SIZE)
        } else {
            (magnitude, dial)
        };

        password += (distance_past_zero + magnitude) / DIAL_SIZE;
        dial = positive_modulo(dial + rotation, DIAL_SIZE);
    }
    println!("{password}");
}
