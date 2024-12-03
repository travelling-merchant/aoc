fn main() {
    // witten dec 2014
    let file: String = String::from("day_02.txt");
    let input: String = std::fs::read_to_string(file).expect("snow snow snow");
    let (r_one, v) = x(&input);
    let r_two = day_two_and_im_aleady_in_despair(v);
    println!("Solution part one = {}", r_one);
    println!("Solution part two = {}", r_two + r_one);
}

fn day_two_and_im_aleady_in_despair(v: Vec<Vec<u8>>) -> u16 {
    let mut total_or_so = 0;
    for vecu in v {
        let is_valid = check_req(&vecu);
        if !is_valid {
            let mut additional_valid = 0;
            for (i, _) in vecu.iter().enumerate() {
                let mut test_v: Vec<_> = vecu.iter().map(|e| e.clone()).collect();
                test_v.remove(i);
                let r = check_req(&test_v);
                if r {
                    additional_valid += 1;
                }
            }
            if additional_valid > 0 {
                total_or_so += 1;
            }
        }
    }

    total_or_so
}

fn x(input: &String) -> (u16, Vec<Vec<u8>>) {
    let mut grand_total: u16 = 0;
    let mut sad_meow: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let mut v: Vec<u8> = Vec::new();
        let nums: Vec<_> = line.split_whitespace().collect();
        for entry in nums {
            let s: String = String::from(entry);
            let p: u8 = s.parse::<u8>().expect("just fuck you lol");
            v.push(p);
        }

        let is_valid = check_req(&v);
        if is_valid == true {
            grand_total += 1;
        }
        sad_meow.push(v);
    }
    (grand_total, sad_meow)
}

fn check_req(v: &Vec<u8>) -> bool {
    let mut is_fine = false;
    let len = v.len();
    let mut is_counter_dec = 0;
    let mut is_counter_inc = 0;

    for (i, entry) in v[..len - 1].iter().enumerate() {
        let n: u8 = v[i + 1];
        let mut diff = 0;

        if *entry > n {
            diff = entry.checked_sub(n).expect("fml");
        } else {
            diff = n.checked_sub(*entry).expect("aaaaaaahhhhh");
        }

        if *entry >= n && diff < 4 && diff != 0 {
            is_counter_dec += 1;
        } else if *entry <= n && diff < 4 && diff != 0 {
            is_counter_inc += 1;
        }
    }
    if is_counter_dec == (len - 1) || is_counter_inc == (len - 1) {
        is_fine = true;
    }
    is_fine
}
