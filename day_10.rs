use std::time::Instant;
fn main() {
    let start = Instant::now();
    let puzzle_input = String::from("3113322113");
    let (result_one, result_two) = loopholes(puzzle_input);
    println!("Solution 1 is = {}", result_one.len());
    println!("Solution 2 is = {}", result_two.len());
    println!("Time taken {:#?}", start.elapsed());
}
fn loopholes(mut input: String) -> (String, String) {
    for _ in 0..40 {
        input = get_adjacent(&input);
    }
    let mut clone_it_yes = input.clone();
    for _ in 0..10 {
        clone_it_yes = get_adjacent(&clone_it_yes);
    }
    (input, clone_it_yes)
}

fn get_adjacent(input: &String) -> String {
    let mut same_n_counter = 1;
    let mut strong: String = String::new();
    let mut basic_counter = 0;
    let mut m = input.chars().skip(1);
    for ch in input.chars() {
        let len_before_end = input.len() - 1;
        if basic_counter < len_before_end {
            if ch == m.next().expect("failuew") {
                same_n_counter += 1;
            } else {
                let amount = format!("{}", same_n_counter);
                strong.push(amount.chars().next().expect("fml"));
                strong.push(ch);
                same_n_counter = 1;
            }
        } else {
            let amount = format!("{}", same_n_counter);
            strong.push(amount.chars().next().expect("fml"));
            strong.push(ch);
        }
        basic_counter += 1;
    }

    strong
}
