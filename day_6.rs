use std::convert::TryInto;
use std::fs;
fn main() {
    // SOME OF MY "FRIENDS" CHALLANGED ME TO DO IT WITH STRINGS
    // don't do this with strings, its a bad idea

    let instructions = fs::read_to_string("day_6.txt".to_string()).expect("no sleep?");
    let grid = create_grid();
    let result = parse_instructions(instructions, grid);
    print!("meow {}", result);
}

fn parse_instructions(input: String, lights: String) -> u32 {
    let mut result = 0;

    // assigns the string to chars
    let mut chars: Vec<char> = lights.chars().collect();
    // for every line in instruction then change things
    for line in input.lines() {
        // get the values splits in reverse
        let half_proccessed: Vec<_> = line.split_whitespace().rev().collect();
        //println!("half {:#?}", half_proccessed);
        let mut v: Vec<usize> = Vec::new();
        // for item in the line vec whos reverssed
        for (i, ele) in half_proccessed.iter().enumerate() {
            // I only care about the 2 entires with number in them
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
                // formula to get the position on a 1000x1000 grid it one line i hope
                let val: u32 = one * 1000 + two;
                // push the position
                v.push(val.try_into().expect("oh no"));
            }
            //          println!("{:#?}", v);
        }
        let mut counter = 0;
        // iter through the range of numbers that need to be changed
        for _ in v[1]..v[0] {
            // check the values of the 3 element
            if half_proccessed[3] == "on" {
                chars[v[1] + counter] = '1';
            } else if half_proccessed[3] == "off" {
                chars[v[1] + counter] = '0';
            } else {
                if chars[v[1] + counter] == '1' {
                    chars[v[1] + counter] = '0';
                } else {
                    chars[v[1] + counter] = '0';
                }
            }
            counter += 1;
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
    let mut grid = String::new();
    while i > 0 {
        grid.push('0');
        i -= 1;
    }
    grid
}
