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

    let mut chars: Vec<char> = lights.chars().collect();
    for line in input.lines() {
        let half_proccessed: Vec<_> = line.split_whitespace().collect();
        let mut v: Vec<u32> = Vec::new();
        for entry in &half_proccessed {
            if entry.contains(',') {
                let nu: u32 = entry
                    .split(',')
                    .next()
                    .expect("fml")
                    .chars()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("coffee in the morning is great");
                let no: u32 = entry
                    .split(',')
                    .last()
                    .expect("fml")
                    .chars()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("coffee in the morning is very great");
                let na = nu * 1000 + no;
                //let na = no * 1000 + nu;
                v.push(na);
            }
        }
        for i in 0..chars.len() {
            if i >= v[0].try_into().expect("nothing") && i <= v[1].try_into().expect("nothing too")
            {
                if half_proccessed[1] == "on" {
                    chars[i] = '1';
                } else if half_proccessed[1] == "off" {
                    chars[i] = '0';
                } else {
                    if chars[i] == '0' {
                        chars[i] = '1';
                    } else {
                        chars[i] = '0';
                    }
                }
            }
        }
    }
    for c in chars {
        if c != '0' {
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
