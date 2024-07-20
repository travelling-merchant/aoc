use std::fs;
#[derive(Debug)]
struct Square {
    square_width: u32,
    square_height: u32,
    square_lenght: u32,
}
fn main() {
    let cubes = read_file_to_square("day_2.txt".to_string());
    let (solution_part_one, gifts_list) = calc_part_one(cubes);
    let sol_p_two = calc_part_two(gifts_list);
    println!("solution part one {:#?}", solution_part_one);
    println!("solution part two {:#?}", sol_p_two);
}

fn calc_part_two(cubes: Vec<Square>) -> u32 {
    let mut t = 0;
    for gift in cubes {
        t += gift.square_lenght * gift.square_width * gift.square_height;
        if gift.square_lenght >= gift.square_width && gift.square_lenght >= gift.square_height {
            t += 2 * gift.square_width + 2 * gift.square_height;
        } else if gift.square_width >= gift.square_lenght && gift.square_width >= gift.square_height
        {
            t += 2 * gift.square_lenght + 2 * gift.square_height;
        } else if gift.square_height >= gift.square_width
            && gift.square_height >= gift.square_lenght
        {
            t += 2 * gift.square_width + 2 * gift.square_lenght;
        } else {
            print!("F {:#?}", gift);
        }
    }

    t
}

fn calc_part_one(cubes: Vec<Square>) -> (u32, Vec<Square>) {
    let mut total = 0;
    for present in &cubes {
        let side_one = present.square_lenght * present.square_width;
        let side_two = present.square_width * present.square_height;
        let side_three = present.square_height * present.square_lenght;
        if side_one <= side_two && side_one <= side_three {
            total += side_one;
        } else if side_two <= side_one && side_two <= side_three {
            total += side_two;
        } else if side_three <= side_two && side_three <= side_one {
            total += side_three;
        }
        total += 2 * side_one + 2 * side_two + 2 * side_three;
    }

    (total, cubes)
}

fn read_file_to_square(filename: String) -> Vec<Square> {
    let file_input = fs::read_to_string(filename).expect("no file idiot");
    let mut gifts: Vec<Square> = Vec::new();
    for line in file_input.lines() {
        let cube = Square {
            square_lenght: line
                .split('x')
                .nth(0)
                .expect("skill isssue")
                .parse()
                .expect("skill issue"),
            square_width: line
                .split('x')
                .nth(1)
                .expect("skill isssue")
                .parse()
                .expect("skill issue"),
            square_height: line
                .split('x')
                .nth(2)
                .expect("skill isssue")
                .parse()
                .expect("skill issue"),
        };
        gifts.push(cube);
    }
    gifts
}
