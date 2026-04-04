fn main() {
    //literally just made this salad to test my c solution but I was wrong twice
    let con = std::fs::read_to_string("day_01.txt").expect("lalal");
    let mut con_as_byte = std::fs::read("day_01.txt").expect("lalal");
    con_as_byte.pop();
    let copy_first = con.chars().nth(0).unwrap();
    let con_2: Vec<_> = format!("{}{}", con, copy_first)
        .chars()
        .filter(|c| *c != '\n')
        .collect();
    let con_2 = con_2.iter().cloned().collect::<String>();
    let mut count: u32 = 0;
    for (index, bytus) in con_2.as_bytes().into_iter().enumerate() {
        if index > 0 && *bytus == con_2.as_bytes()[index - 1] {
            count += (con.as_bytes()[index - 1] - 48) as u32;
        }
    }
    let half = con_as_byte.len() / 2;
    let mut part_two: u32 = 0;
    for entry in 0..half {
        if con_as_byte[entry] == con_as_byte[entry + half] {
            part_two += (con_as_byte[entry] - 48) as u32;
            part_two += (con_as_byte[entry + half] - 48) as u32;
        }
    }
    println!("Solution part one = {}", count);
    println!("Solution part two = {}", part_two);
}
