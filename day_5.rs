use std::fs; fn main(){ let file_name = String::from("day_5.txt");
let file_content = fs::read_to_string(file_name)
.expect("who do you think you are, just tryint to run this without provising a input");
let inspected_file = string_inspector(file_content);
println!("Solution to part one is {}",inspected_file);
}
fn string_inspector(input:String)->u32{
let mut result = 0;
let vowels = ['a','e','i','o','u'];
	for line in input.lines(){
	let mut vowels_counter = 0;
	let mut double_letter_counter = 0;
	let mut bad_words_counter= 0;
		
		for (i,c) in line.chars().enumerate(){
			if vowels.contains(&c){
			vowels_counter +=1;
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
