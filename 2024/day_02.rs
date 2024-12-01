fn main() {
    // witten dec 2014
    let file: String = String::from("day_02.txt");
    let input: String = std::fs::read_to_string(file).expect("snow snow snow");
    let r_one = x(&input);
    println!("Solution part one = {}", r_one);
}

fn x(input: &String) -> u16 {
    666
}
