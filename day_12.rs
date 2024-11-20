const PUZZLE_DATA:&str = "day_12.txt";
fn main(){
let data = std::fs::read_to_string(PUZZLE_DATA.to_string()).expect("didn't you forgot something?");	
let m = sum_numbers(&data.as_bytes());
println!("data lenght {}",m);
}

fn sum_numbers(data:&[u8])->u32{
let mut total_sum:u32 = 0;
let ascii_num:[u8;10] = [48,49,50,51,52,53,54,55,56,57];
let ascii_minus = 45;
let mut is_minus:bool= false;
let mut last_symbol:u8= 0;
let mut intermediate_num = 0;
let mut itoratoru = data.iter();
	itoratoru.next();
	let len = data.len()-1;
	for byte in data[0..len].iter(){
				println!("{}",byte);
		let next = itoratoru.next();
		if ascii_num.contains(byte) && ascii_num.contains(next.expect("the fire has gone out")){
			if last_symbol != 45 {
			last_symbol = *byte;
			}
			else{
			is_minus = true;
			}
			let b:u32 = *byte as u32;
			intermediate_num = b;
			intermediate_num -= 48;
			intermediate_num *= 10;
			intermediate_num += *next.expect("but I wasn't prepared to light a new one")  as u32;
			intermediate_num -= 48;
		}
		else if ascii_num.iter().any( |n| n == byte){
			// check previous for - symbol
			if intermediate_num != 0{
				intermediate_num *= 10;
				intermediate_num += *byte as u32;
				intermediate_num -= 48;
				if is_minus == true{
				total_sum -= intermediate_num as u32;
				is_minus = false
				}else{
				total_sum += intermediate_num as u32;
				}
				intermediate_num = 0;

			}
			else{
				println!("{}",byte);
// if minus
				if is_minus == true{
	   			total_sum -= *byte as u32;
	   			total_sum +=48;
				}else{
	   			total_sum += *byte as u32;
	   			total_sum -=48;
				}
			}
		}
	}
total_sum
}
