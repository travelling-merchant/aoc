use std::ops::Range;

fn main() {
    let file = std::fs::read_to_string("day_05.txt").expect("wheres the catch");
    let banana_split: Vec<&str> = file.split("\n\n").collect();
    let (result, other_result) = run_over_night(banana_split);
    println!("Solution part one = {:#?}", result);
    println!("Solution part two = {:#?}", other_result);
}

fn run_over_night(mut input: Vec<&str>) -> (u16, u64) {
    let mut fresh: u16 = 0;
    let numbers: Vec<u64> = input
        .pop()
        .expect("lol")
        .lines()
        .map(|line| line.parse::<u64>().expect("you where alone the whole time"))
        .collect();

    let ranges: Vec<Range<u64>> = input
        .pop()
        .expect("❌")
        .lines()
        .map(|line| Range::<u64> {
            start: line
                .split('-')
                .nth(0)
                .expect("parse unknonw")
                .parse::<u64>()
                .expect("nothing"),
            end: line
                .split('-')
                .nth(1)
                .expect("this is a mess")
                .parse::<u64>()
                .expect("???")
                + 1,
        })
        .collect();

    for number in numbers.iter() {
        for range in ranges.iter() {
            if range.contains(&number) {
                fresh += 1;
                break;
            }
        }
    }
    let mut better_than_range: Vec<[u64; 2]> = Vec::new();
    for range in ranges.iter() {
        better_than_range.push([range.start, range.end - 1]);
    }
    better_than_range.sort_by(|s, o| {
        if s[0] == o[0] {
            s[1].cmp(&o[1])
        } else {
            s[0].cmp(&o[0])
        }
    });

    let mut test: Vec<[u64; 2]> = Vec::new();
    for range in better_than_range.iter() {
        if let Some(last_one) = test.last_mut() {
            if range[0] <= last_one[1] {
                if range[1] > last_one[1] {
                    last_one[1] = range[1];
                }
                continue;
            }
        }

        test.push(*range);
    }
    let total_destruction = test.iter().map(|e| e[1] - e[0] + 1).sum();

    (fresh, total_destruction)
}
