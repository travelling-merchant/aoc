const PUZZLE_DATA: &str = "day_12.txt";
fn main() {
    test_sum_func_and_dance();
    let data =
        std::fs::read_to_string(PUZZLE_DATA.to_string()).expect("didn't you forgot something?");
    let m = sum_numbers(&data.as_bytes());
    println!("data lenght {}", m);
}

fn sum_numbers(data: &[u8]) -> i32 {
    let mut total_sum: i32 = 0;
    let ascii_num: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
    let ascii_minus = 45;
    let mut is_minus: bool = false;
    let mut last_symbol: u8 = 0;
    let mut intermediate_num = 0;
    let mut itoratoru = data.iter();
	let mut count = 0;
    itoratoru.next();
    let len = data.len() - 1;
    for byte in data[0..len].iter() {
        println!("{}", byte);
        let next = itoratoru.next();
        if ascii_num.contains(byte) && ascii_num.contains(next.expect("the fire has gone out")) {
            if last_symbol != ascii_minus {
                last_symbol = *byte;
            } else {
				println!("last symbol {}", last_symbol);
                is_minus = true;
            }
            let b: i32 = *byte as i32;
			if intermediate_num != 0{
            intermediate_num = b;}
			else{
            intermediate_num += b;
			}
            intermediate_num -= 48;
            intermediate_num *= 10;
            intermediate_num += *next.expect("but I wasn't prepared to light a new one") as i32;
            intermediate_num -= 48;
        } else if ascii_num.iter().any(|n| n == byte) {
			println!("is executed bitch");
            if intermediate_num != 0 {
                if last_symbol == ascii_minus {
                    is_minus = true;
                }
                last_symbol = *byte;
                intermediate_num *= 10;
                intermediate_num += *byte as i32;
                intermediate_num -= 48;
                if is_minus == true {
                    total_sum -= intermediate_num as i32;
                    is_minus = false
                } else {
                    total_sum += intermediate_num as i32;
                }
                intermediate_num = 0;
            } else {
				println!("count {}",count);
				count+=1;
                if last_symbol == ascii_minus {
                    is_minus = true;
                }
				println!("last symbol {}",last_symbol);
                last_symbol = *byte;
                if is_minus == false {
					println!("DUCK ");
                    total_sum += *byte as i32;
                    total_sum -= 48;
                } else {
					println!("FUCK FUCK FUCK");
                    total_sum += 48;
                    total_sum -= *byte as i32;
                }
            }
		}
            last_symbol = *byte;
				if last_symbol == 45{
		is_minus = true;	
				}
        
    }
    total_sum
}
fn test_sum_func_and_dance() {
    println!("testing sum function");
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

    let data = "{441}";
    let expected_result = 441;
    let result_func = sum_numbers(&data.as_bytes());
    assert_eq!(
        result_func, expected_result,
        "test 5 failed with data {} = {}",
        result_func, expected_result
    );
println!("test complete");
}
