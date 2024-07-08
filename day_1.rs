use std::fs;
fn main(){
let mut start_floor = 0;
let mut s_p = 0;
let mut s_p_2 = 0;
let input = fs::read_to_string("day_1.txt").expect("you do not have file lol");
let mut v = Vec::new();
for entry in input.chars(){
		s_p +=1;
		match entry{
		 '(' => {start_floor +=1; if start_floor < 0 {v.push(s_p)};},
		')' => {start_floor -=1 ;if start_floor < 0 {v.push(s_p)};},
		_   => print!(""),
		};
		
		
	} 
		s_p_2 += v[0];
	println!(" Solution to part 1 is {start_floor}");
	println!(" Solution to part 2 is {s_p_2}");
}
