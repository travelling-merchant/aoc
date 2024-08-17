use std::fs;
fn main() {
    let r = p_o();
    println!("Solution part one {r}");
}
fn p_o() -> usize {
    let input = fs::read_to_string("day_8.txt").expect("I'm good at waisting time");
    let mut start_lenght = 0;
    let mut amazing_vec_uwu: Vec<char> = Vec::new();
    // needs to be done for each line to not get \n as extra char in the count
    for line in input.lines() {
        start_lenght += line.len();

        let mut evil_i = 0;
        let mut skip_counter = 0;
        for c in line.chars() {
            if evil_i > 0 && evil_i < line.len() - 1 {
                if skip_counter < 1 {
                    let following = line.chars().nth(evil_i + 1).expect("whatever");
                    if c == '\\' && following == c {
                        amazing_vec_uwu.push(following);
                        skip_counter += 1;
                    } else if c == '\\' && following == '\"' {
                        amazing_vec_uwu.push(following);
                        skip_counter += 1;
                    } else if c == '\\' && following == 'x' {
                        let s_ascii = format!(
                            "{}{}",
                            line.chars()
                                .nth(evil_i + 2)
                                .expect("there must be a better way"),
                            line.chars().nth(evil_i + 3).expect("but me lazy")
                        );
                        let ascii =
                            u8::from_str_radix(&s_ascii, 16).expect("haha you failed once again");
                        amazing_vec_uwu.push(ascii as char);
                        skip_counter += 3;
                    } else {
                        amazing_vec_uwu.push(c);
                    }
                } else {
                    skip_counter -= 1;
                }
            }
            evil_i += 1;
        }
    }

    let mem_len = amazing_vec_uwu.len();
    start_lenght - mem_len
}
