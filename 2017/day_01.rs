fn main() {
    //literally just made this salad to test my c solution but I was wrong twice
    let con = std::fs::read_to_string("day_01.txt").expect("lalal");
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
    println!("Solution part one = {}", count);
}
