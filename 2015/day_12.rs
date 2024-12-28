// written in november 2024
use std::ops::Range;
use std::time::Instant;
const PUZZLE_DATA: &str = "day_12.txt";
const ASCII_NUM: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
fn main() {
    let start = Instant::now();
    test_sum_func_and_dance();
    let data =
        std::fs::read_to_string(PUZZLE_DATA.to_string()).expect("didn't you forgot something?");
    let data_2 =
        std::fs::read_to_string("test.txt".to_string()).expect("didn't you forgot something?");
    let r_one = sum_numbers(&data.as_bytes());
    let pre_proccessed_r_two = censor_red(data);
    let r_two = sum_numbers(&pre_proccessed_r_two.as_bytes());
    println!("Solution part one = {}", r_one);
    println!(
        "Solution part two (a wrong answer that is to high is [74780]), current answer = {}",
        r_two
    );
    println!("Time taken {:#?}", start.elapsed());
}

fn censor_red(mut data: String) -> String {
    //let len = data.len();
    //for (index,chars) in data[..=len-2].chars().collect::<Vec<_>>().windows(3).enumerate(){
    //	let mut maybe_red = chars[0.2].collect();
    //	if chars == "red"{
    //		println!("yuhhe");
    //	}
    //}

    let mut ranger_platoon: Vec<Range<usize>> = Vec::new();
    let something: Vec<_> = data.match_indices("red").map(|value| value.0).collect();
    for thing in something.iter() {
        let ending = race_to_the_end(*thing, &data);
        println!("some endings {:#?}", ending);
        match ending {
            Some(ending) => {
                let start = after_the_end_is_the_start(*thing, &data);
                let ranger: Range<usize> = Range {
                    start: start,
                    end: ending,
                };
                ranger_platoon.push(ranger);
            }
            None => (),
        }
    }

	println!("ranger or so{:#?}",ranger_platoon);
    let something :String= data
        .chars()
        .enumerate()
        .filter(|i| {
            ranger_platoon
                .iter()
                .map(|rawr| !rawr.contains(&i.0))
                .reduce(|what, ever| what && ever)
                .expect("you going to blow up?")
        })
        .map(|i| i.1)
        .collect::<String>();
    print!(" the print at the very end {}", something);
    something
}

fn after_the_end_is_the_start(thing: usize, data: &String) -> usize {
    let mut index_counter = thing.clone();
    let staring_pos = data[0..=thing-1].chars().rev();
    let scan_result: (char, usize) = staring_pos
        .scan((1, 1), |state, x| {
            index_counter -= 1;
            match x {
                '}' => state.0 += 1,
                ']' => state.1 += 1,
                '[' => state.1 -= 1,
                '{' => state.0 -= 1,
                _ => (),
            }
            if state.0 == 0 {
                return None;
            } else if state.1 == 0 {
                return None;
            } else {
                return Some((x, index_counter));
            }
        })
        .last()
        .expect("someone belives its always something uwu");
    scan_result.1;
	println!("should be a number {}",scan_result.1);
scan_result.1
}

fn race_to_the_end(thing: usize, data: &String) -> Option<usize> {
    let mut index_counter = thing.clone();
    let staring_pos = data.trim().chars().skip(thing  );
	let _= staring_pos.clone().inspect(|e|println!("in theory {:#?}",e));
	let mut is_suqare = false;
    let scan_result: (char, usize) = staring_pos.inspect(|e|println!("in theory {}",e))
        .scan((1, 1), |state, x| {
            index_counter += 1;
            match x {
                '}' => state.0 -= 1,
                ']' => state.1 -= 1,
                '[' => state.1 += 1,
                '{' => state.0 += 1,
                _ => (),
            }
            if state.0 == 0 {
                return None;
            } else if state.1 == 0 {
				is_suqare = true;
                return None;
            } else {
                return Some((x, index_counter));
            }
        })
        .last()
        .expect("someone belives its always something uwu");
    if !is_suqare {
println!("scan res {:#?}",scan_result);
        return Some(scan_result.1);
    }
    None
}

fn sum_numbers(data: &[u8]) -> i32 {
    let mut total_sum: i32 = 0;
    let mut initialze_bool: bool = false;
    let mut intermed_num: Option<[Option<&u8>; 3]> = None;
    let mut next_symbol = data.iter();
    next_symbol.next();
    let len = data.len() - 1;
    for byte in data[0..len].iter() {
        let nexu_symb_u8: &u8 = next_symbol.next().expect("you fool");
        let (current_bool, current_intermed, number_to_add) =
            get_num_or_intermediate_num(initialze_bool, intermed_num, byte, nexu_symb_u8);
        initialze_bool = current_bool;
        intermed_num = current_intermed;
        match number_to_add {
            Some(n) => {
                total_sum = total_sum + n;
            }
            _ => (),
        }
    }
    total_sum
}

fn get_num_or_intermediate_num<'a>(
    mut is_minus: bool,
    mut intermed_num: Option<[Option<&'a u8>; 3]>,
    current_symb: &'a u8,
    next_symb: &'a u8,
) -> (bool, Option<[Option<&'a u8>; 3]>, Option<i32>) {
    let ascii_minus: u8 = 45;
    let ascii_number_start: u8 = 48;
    let mut number_to_add: Option<i32> = None;

    if is_minus == false && current_symb == &ascii_minus {
        is_minus = true;
    }
    if ASCII_NUM.contains(current_symb) {}
    if ASCII_NUM.contains(current_symb) && ASCII_NUM.contains(next_symb) {
        match intermed_num {
            Some(mut exists) => {
                for e in exists.iter_mut() {
                    if e.is_none() {
                        *e = Some(current_symb);
                        break;
                    }
                }
                intermed_num = Some(exists)
            }
            None => {
                intermed_num = Some([Some(current_symb), None, None]);
            }
        };
    } else if ASCII_NUM.contains(current_symb) {
        let mut is_last_element = 0;
        match intermed_num {
            Some(mut awray) => {
                for a in awray {
                    if a.is_some() {
                        is_last_element += 1;
                    }
                }
                awray[2] = Some(current_symb);
                number_to_add = Some(0);
                for last_entry in 0..=2 {
                    let mut return_number =
                        number_to_add.expect("if you see this massge printed, you know you're bad");
                    match awray[last_entry] {
                        Some(entry) => {
                            return_number += *entry as i32;
                            return_number -= ascii_number_start as i32;
                            number_to_add = Some(return_number);
                        }
                        None => (),
                    }

                    if is_last_element > 0 {
                        is_last_element -= 1;
                        match number_to_add {
                            Some(n) => number_to_add = Some(n * 10),
                            _ => continue,
                        };
                    }
                }
                intermed_num = None;
            }
            None => number_to_add = Some(*current_symb as i32 - ascii_number_start as i32),
        }
        if is_minus == true {
            match number_to_add {
                Some(nunu) => {
                    number_to_add = Some(nunu * -1);
                }
                None => (),
            }
        }
        is_minus = false;
    }
    (is_minus, intermed_num, number_to_add)
}

fn test_sum_func_and_dance() {
    println!("executing tests sum function...");
    let data = "{w\"a\":2,\"b\":4}";
    let expected_result = 6;

    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 0 failed with data {} = {}",
        result_func, expected_result
    );

    let result_func = sum_numbers(&[91, 49, 44, 50, 44, 51, 93]);
    assert_eq!(
        result_func, expected_result,
        "test 1 failed with data {} = {}",
        result_func, expected_result
    );

    let data = "[[[3]]]";
    let expected_result = 3;

    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 2 failed with data {} = {}",
        result_func, expected_result
    );

    let data = "{\"a\":{\"b\":4},\"c\":-1}";
    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 3 failed with data {} = {}",
        result_func, expected_result
    );

    let data = "[]";
    let expected_result = 0;

    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 4 failed with data {} = {}",
        result_func, expected_result
    );

    let data = "{-451}";
    let expected_result = -451;
    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 5 failed with data {} = {}",
        result_func, expected_result
    );
    let data = "{-51}";
    let expected_result = -51;
    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 6 failed with data {} = {}",
        result_func, expected_result
    );
    println!("tests complete!");
    println!("");
}
