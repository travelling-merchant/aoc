fn main() {
    let input =
        std::fs::read_to_string("day_03.txt".to_string()).expect("why cant I keep it simple");
    let vec_with_instructions = get_math_oh_no(&input);
    let result_one: u32 = find_your_inner_math_saint(&vec_with_instructions);
    println!("Solution part one = {}", result_one);
}

fn find_your_inner_math_saint(vec_m: &Vec<String>) -> u32 {
    let mut ultime_math_operation: u32 = 0;
    for entry in vec_m {
        let better_entry = entry.split('(').last();
        let better_better_entry = better_entry
            .expect("but why are we doing this")
            .split(')')
            .next();
        let better_one: u32 = better_better_entry
            .expect("right, to forget")
            .split(',')
            .next()
            .expect("about the bad things")
            .parse()
            .expect("cheers");
        let better_two: u32 = better_better_entry
            .expect("keep on keeping on")
            .split(',')
            .last()
            .expect("spin right round")
            .parse()
            .expect("just like a planet or a sun");
        ultime_math_operation += better_one * better_two;
        println!("better entry = {:#?}", better_better_entry);
        println!("better one = {:#?}", better_one);
        println!("better two = {:#?}", better_two);
    }
    ultime_math_operation
}

fn get_math_oh_no(input: &String) -> Vec<String> {
    let search_pattern = String::from("mul(");
    let mut data = input.clone();
    let mut math_i_think: Vec<String> = Vec::new();
    data.push_str("looking for some cozy places uwu");
    for (i, _) in data.chars().enumerate() {
        if data[i..].starts_with("mul(") {
            let mut end_of_mul = 0;
            let maximum_pos_mul = 12;

            for (close_bracket_pos, wow) in data[i..i + maximum_pos_mul].chars().enumerate() {
                if wow == ')' && close_bracket_pos < maximum_pos_mul {
                    end_of_mul = close_bracket_pos;
                }
            }
            if end_of_mul == 0 {
                continue;
            }

            let paw = data[i..].chars().take(end_of_mul + 1).collect();
            math_i_think.push(paw);
        }
    }

    math_i_think = math_i_think
        .into_iter()
        .filter(|e| e.contains(","))
        .collect();
    println!(" data I hope {:#?}", math_i_think);
    math_i_think
}
