use std::fs;
fn main() {
    let file_name = String::from("day_5.txt");
    let file_content = fs::read_to_string(file_name)
        .expect("who do you think you are, just tryint to run this without provising a input");
    let p_2 = p_two(&file_content);
    let inspected_file = p_one(file_content);
    println!("Solution to part one is {}", inspected_file);
    println!("Solution to part two is {}", p_2);
}
fn p_two(input: &String) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let len = line.len();
        let mut coffee = false;
        let mut cookie = false;
        let char_v: Vec<char> = line.chars().collect();
        for i in 0..len - 2 {
            if char_v[i] == char_v[i + 2] {
                coffee = true;
                break;
            }
        }
        let mut symbols: Vec<String> = Vec::new();
        for (i, entry) in line.chars().enumerate() {
            if i < len - 1 {
                let d = format!("{}{}", entry, line.chars().nth(i + 1).expect("fml"));
                symbols.push(d);
            }
        }

        for (i, c) in line.chars().enumerate() {
            if i < len - 3 {
                let key = format!(
                    "{}{}",
                    line.chars().nth(i).expect("sheeps are moving"),
                    line.chars().nth(i + 1).expect("but d cohi is empty")
                );
                if symbols[i + 2..len - 1].contains(&key) {
                    cookie = true;
                }
            }
        }

        if coffee == true && cookie == true {
            result += 1;
        }
    }

    result
}

fn p_one(input: String) -> u32 {
    let mut result = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for line in input.lines() {
        let mut vowels_counter = 0;
        let mut double_letter_counter = 0;
        let mut bad_words_counter = 0;

        for (i, c) in line.chars().enumerate() {
            if vowels.contains(&c) {
                vowels_counter += 1;
            }
            if i > 0 {
                if c == line.chars().nth(i - 1).expect("it just cant be helped") {
                    double_letter_counter += 1
                }
            }
        }
        if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
        {
            bad_words_counter += 1;
        }
        if vowels_counter >= 3 && bad_words_counter <= 0 && double_letter_counter > 0 {
            result += 1;
        }
    }
    result
}
