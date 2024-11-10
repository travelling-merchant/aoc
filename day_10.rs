use std::primitive::char;
fn main(){
let puzzle_input = String::from("3113322113");
let result_one = loopholes(puzzle_input);
println!("Solution 1 is = {}",result_one.len());
}
fn loopholes(mut input:String)->String{
let mut counter = 0;	
	for _ in 0..40{
	counter +=1;
	println!("counter  is {}",counter);
	//println!("input is {}",&input);
	input = get_adjacent(&input);
	}
input
}


fn get_adjacent(input:&String)->String{
let mut same_n_counter:u8 = 1;

let mut strong:String= String::new();

for (i,char) in input.chars().enumerate(){
	let len_before_end = input.len() -1;
	if i < len_before_end{
		if char == input.chars().nth(i+1).expect("could or could not"){
			same_n_counter +=1;	
		}
		else {
			let amount= format!("{}",same_n_counter);
			strong.push(amount.chars().nth(0).expect("fml"));
			strong.push(char);
			same_n_counter = 1;
		
		}
	}
	else {
			let amount= format!("{}",same_n_counter);
			strong.push(amount.chars().nth(0).expect("fml"));
			strong.push(char);
			}
		
	
}

strong
}
