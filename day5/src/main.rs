use std::{cmp::Ordering, fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let (ranges_str, available_ids) = input.split_once("\n\n").unwrap();

    // Part 1
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    for fr in ranges_str.lines() {
        let (start, end) = fr.split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.push(start..=end);
    }
    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let ranges = merged_ranges(&ranges);

    let mut result = 0;
    for id in available_ids.lines() {
        let id: u64 = id.parse().unwrap();
        let is_fresh = ranges
            .binary_search_by(|r| {
                if *r.end() < id {
                    Ordering::Less
                } else if *r.start() > id {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .is_ok();

        result += if is_fresh { 1 } else { 0 };
    }

    println!("{result}");

    // Part 2
    let all_fresh_count: u64 = ranges.iter().map(|r| 1 + r.end() - r.start()).sum();
    println!("{all_fresh_count}");
}

fn merged_ranges(sorted_ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let Some(mut current_range) = sorted_ranges.first().cloned() else {
        return vec![];
    };

    let mut result = vec![];
    for range in sorted_ranges.iter() {
        if current_range.contains(range.start()) {
            current_range = range_union(&current_range, range)
        } else {
            result.push(current_range);
            current_range = range.clone();
        }
    }
    result.push(current_range);

    result
}

fn range_union(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    (*r1.start().min(r2.start()))..=(*r1.end().max(r2.end()))
}
