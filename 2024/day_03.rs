fn main() {
    // written in dec 2024
    let input =
        std::fs::read_to_string("day_03.txt".to_string()).expect("why cant I keep it simple");
    let vec_with_whatever = to_do_or_not_to_do(input);
    let (result_one, result_two) = find_your_inner_math_saint(vec_with_whatever);
    println!("Solution part one = {}", result_one);
    println!("Solution part two = {}", result_two);
}

fn find_your_inner_math_saint(do_do_not: Vec<String>) -> (u32, u32) {
    let mut ultimate_math_operation: u32 = 0;
    let mut weak_stuff: u32 = 0;
    let mut stop_it = false;
    for entry in do_do_not {
        if entry.starts_with("don't") {
            stop_it = true;
        } else if entry.starts_with("do()") {
            stop_it = false;
        } else if entry.contains(",") {
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
            ultimate_math_operation += better_one * better_two;
            if !stop_it {
                weak_stuff += better_one * better_two;
            }
        }
    }
    (ultimate_math_operation, weak_stuff)
}

fn to_do_or_not_to_do(mut input: String) -> Vec<String> {
    let mut dodododo: Vec<String> = Vec::new();
    input.push_str("why is it dark every time I look out the window?");
    for (i, _) in input.chars().enumerate() {
        if input[i..].starts_with("mul(")
            || input[i..].starts_with("don't()")
            || input[i..].starts_with("do()")
        {
            let mut end_of_value = 0;
            let maximum_pos_mul = 12;

            for (end_of_ins, entry) in input[i..i + maximum_pos_mul].chars().enumerate() {
                if entry == ')' && end_of_value < maximum_pos_mul {
                    end_of_value = end_of_ins;
                }
            }
            if end_of_value == 0 {
                continue;
            }
            let instruction = input[i..].chars().take(end_of_value + 1).collect();
            dodododo.push(instruction);
        }
    }
    dodododo
}
