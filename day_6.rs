use std::convert::TryInto;
use std::fs;
fn main() {
    // Some friends wannted that I make it with strings.
    // bad idea, and anywhay I still eneded up having vecs
    // but maybe it is possible to only have one string with ascii who knows no time maybe I will come back later
    let instructions = fs::read_to_string("day_6.txt".to_string()).expect("no sleep?");
    let grid = create_grid();
    let result_one = part_one(&instructions, &grid);
    let result_two = part_two(instructions, grid);
    println!("Solution part one {}", result_one);
    println!("Solution part two {}", result_two);
}

fn part_two(input: String, lights: String) -> u64 {
    let mut result = 0;

    let meows: Vec<char> = lights.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for meow in meows {
        if meow != '\n' {
            chars.push(meow);
        }
    }
    for line in input.lines() {
        let half_proccessed: Vec<_> = line.split_whitespace().rev().collect();
        let mut v: Vec<usize> = Vec::new();
        for (i, ele) in half_proccessed.iter().enumerate() {
            if i == 0 || i == 2 {
                let one: u32 = ele
                    .split(',')
                    .nth(0)
                    .expect("this number")
                    .parse()
                    .expect("getting sick");
                let two: u32 = ele
                    .split(',')
                    .nth(1)
                    .expect("whats a number")
                    .parse()
                    .expect("is part of the human exp");
                v.push(one.try_into().expect("oh no"));
                v.push(two.try_into().expect("oh no"));
            }
        }
        for (i, row) in lights.lines().enumerate() {
            if i >= v[2] && i <= v[0] {
                for j in 0..row.len() {
                    if j >= v[3] && j <= v[1] {
                        let get_v_index: usize = i * 1000 + j;
                        if half_proccessed[3] == "on" {
                            let ascii = chars[get_v_index] as u8 + 1;
                            if ascii == 127 {
                                println!("fuck");
                            }
                            chars[get_v_index] = ascii as char;
                        } else if half_proccessed[3] == "off" {
                            if chars[get_v_index] != '0' {
                                let ascii = chars[get_v_index] as u8 - 1;
                                if ascii == 127 {
                                    println!("fuck");
                                }
                                chars[get_v_index] = ascii as char;
                            }
                        } else {
                            let ascii = chars[get_v_index] as u8 + 2;
                            if ascii == 127 {
                                println!("fuck");
                            }
                            chars[get_v_index] = ascii as char;
                        }
                    }
                }
            }
        }
    }
    for c in chars {
        // - 48 because ascii value '0' is 48 '1' 49 etc bitch and merry christmas
        let num = c as u8 - 48;
        result += num as u64;
    }
    result
}

fn part_one(input: &String, lights: &String) -> u32 {
    let mut result = 0;

    let meows: Vec<char> = lights.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for meow in meows {
        if meow != '\n' {
            chars.push(meow);
        }
    }
    for line in input.lines() {
        let half_proccessed: Vec<_> = line.split_whitespace().rev().collect();
        let mut v: Vec<usize> = Vec::new();
        for (i, ele) in half_proccessed.iter().enumerate() {
            if i == 0 || i == 2 {
                let one: u32 = ele
                    .split(',')
                    .nth(0)
                    .expect("this number")
                    .parse()
                    .expect("getting sick");
                let two: u32 = ele
                    .split(',')
                    .nth(1)
                    .expect("whats a number")
                    .parse()
                    .expect("is part of the human exp");
                v.push(one.try_into().expect("oh no"));
                v.push(two.try_into().expect("oh no"));
            }
        }
        for (i, row) in lights.lines().enumerate() {
            if i >= v[2] && i <= v[0] {
                for j in 0..row.len() {
                    if j >= v[3] && j <= v[1] {
                        let get_v_index: usize = i * 1000 + j;
                        if half_proccessed[3] == "on" {
                            chars[get_v_index] = '1';
                        } else if half_proccessed[3] == "off" {
                            chars[get_v_index] = '0';
                        } else {
                            if chars[get_v_index] == '0' {
                                chars[get_v_index] = '1'
                            } else {
                                chars[get_v_index] = '0'
                            }
                        }
                    }
                }
            }
        }
    }
    for c in chars {
        if c == '1' {
            result += 1;
        }
    }
    result
}

fn create_grid() -> String {
    let mut i = 1000000;
    let mut ii = 0;
    let mut grid = String::new();
    while i > 0 {
        ii += 1;
        if ii > 999 {
            grid.push('\n');
            ii = 0;
        }
        grid.push('0');
        i -= 1;
    }
    grid
}
