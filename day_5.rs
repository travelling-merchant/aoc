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
	for line in input.lines(){
		let len = line.len();
		let mut coffee = false;
		let mut cookie = false;
		let char_v:Vec<char> = line.chars().collect();
		for  i in 0..len-2{ 
			if char_v[i] == char_v[i+2]{
				coffee = true;
				break;
				}
		}
		let mut for_line = 0;
		for i in 0..len-3{
				let mut count_1 = 0;
			for c in line.chars(){
				if count_1 >= len-3{break;}
				if c  == char_v[count_1+2] && line.chars().nth(i+1).expect("ITS WEEKEND BITCH") == char_v[count_1+3]{
					for_line +=1;
					break;
					}
				count_1 +=1;
			}
		}
			if for_line > 0{cookie = true;}
		if coffee == true && cookie == true{
			result+=1;
			}
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
