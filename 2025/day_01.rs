fn main() {
    let f = std::fs::read_to_string("day_01.txt".to_string())
        .expect("no sleep, but stress, because people");
    let (result_one, result_two) = work_sleep_aaaaaah(f);
    println!("Solution part one = {}", result_one);
    println!("Solution part two = {}", result_two);
}
fn work_sleep_aaaaaah(input: String) -> (u16, u16) {
    let mut result: u16 = 0;
    let mut result_two: u16 = 0;

    let mut counter: i32 = 50;
    let mut uhm = false;

    for line in input.lines() {
        if line.chars().nth(0).unwrap() == 'L' {
            uhm = false;
        }
        if line.chars().nth(0).unwrap() == 'R' {
            uhm = true;
        }
        let num = line.get(1..).unwrap().parse::<i32>().unwrap();
        let (cou, res, res_two) = fucked_counter(counter, num, uhm);
        counter = cou;
        result += res;
        result_two += res_two;
    }
    (result, result_two)
}
fn fucked_counter(mut counter: i32, num: i32, b: bool) -> (i32, u16, u16) {
    let mut n: u16 = 0;
    let mut nn: u16 = 0;
    if !b {
        for _ in 0..num {
            counter -= 1;

            if counter == -1 {
                counter = 99;
            }
            if counter == 0 {
                nn += 1;
            }
        }
    } else {
        for _ in 0..num {
            counter += 1;
            if counter == 100 {
                counter = 0;
            }
            if counter == 0 {
                nn += 1;
            }
        }
    }
    if counter == 0 {
        n += 1;
    }
    (counter, n, nn)
}
