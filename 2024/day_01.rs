fn main() {
    // written dec 2024
    let input = std::fs::read_to_string("day_01.txt".to_string()).expect("go to bed");
    let (r_one, v_one, v_two) = get_p_one(&input);
    let r_two = get_p_two(v_one, v_two);
    println!("Solution part one = {}", r_one);
    println!("Solution part two = {}", r_two);
}

fn get_p_two(v_one: Vec<u32>, v_two: Vec<u32>) -> u32 {
    let mut amazing_total = 0;
    for entry in v_one.iter() {
        let r: u32 = v_two.iter().filter(|&e| e == entry).count() as u32;
        let rr: u32 = entry * r;
        amazing_total += rr;
    }
    amazing_total
}
fn get_p_one(input: &String) -> (u32, Vec<u32>, Vec<u32>) {
    let mut sum_p_one = 0;
    let mut v_one: Vec<u32> = Vec::new();
    let mut v_two: Vec<u32> = Vec::new();
    for line in input.lines() {
        let p_one = line.split_whitespace().next();
        let p_two = line.split_whitespace().last();
        v_one.push(p_one.expect("coffee").parse::<u32>().unwrap());
        v_two.push(p_two.expect("pls").parse::<u32>().unwrap());
    }

    v_one.sort();
    v_two.sort();

    for i in 0..=999 {
        if v_one[i] > v_two[i] {
            let r = v_one[i] - v_two[i];
            sum_p_one += r;
        } else {
            let r = v_two[i] - v_one[i];
            sum_p_one += r;
        }
    }
    (sum_p_one, v_one, v_two)
}
