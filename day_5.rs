use std::fs; fn main(){ let file_name = String::from("day_5.txt");
let file_content = fs::read_to_string(file_name)
.expect("who do you think you are, just tryint to run this without provising a input");
let p_2 = p_two (&file_content);
let inspected_file = p_one(file_content);
println!("Solution to part one is {}",inspected_file);
println!("Solution to part two is {}",p_2);
}
fn p_two(input:&String)->u32{
let mut result = 0;
let mut good_words = 0;
let mut double_double = 0;
	for line in input.lines(){
	let len = line.len();
		for (i,c) in line.chars().enumerate(){
			if i < len -2{
				if c == line.chars().nth(i+2).expect("you failed, just one more time"){
					good_words +=1;
					}
			
				}
				let mut counter = 0;
				let mut line_track= 0;
				while counter <= len -3 && line_track == 0{
							if c == line.chars().nth(counter+2).expect("rainy day")
							&& line.chars().nth(counter+1) == line.chars().nth(counter+3){
							println!("{:?}{:?}",line.chars().nth(counter +1),line.chars().nth(counter +3));
					 		double_double +=1;
							line_track +=1;
							}
				counter +=1
				}
			}
			if good_words > 0 && double_double > 0{
				result +=1;
				}
			good_words = 0;
			double_double = 0;

		

	}
result
}

fn p_one(input:String)->u32{
let mut result = 0;
let vowels = ['a','e','i','o','u'];
	for line in input.lines(){
	let mut vowels_counter = 0;
	let mut double_letter_counter = 0;
	let mut bad_words_counter= 0;
		
		for (i,c) in line.chars().enumerate(){
			if vowels.contains(&c){ vowels_counter +=1;
			}
			if i > 0{
			if c == line.chars().nth(i-1).expect("it just cant be helped"){double_letter_counter +=1}
			}
		}
		if line.contains("ab")||line.contains("cd")|| line.contains("pq") || line.contains("xy"){bad_words_counter +=1;}
		if vowels_counter >= 3 && bad_words_counter <= 0 && double_letter_counter > 0{
		result +=1;
		}
	}
result
}
