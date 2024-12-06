use std::collections::HashMap;
const XMAS:&str = "XMAS";
const DRUNK_XMAS:&str = "SAMX";

// writen in dec 2024

#[derive(Debug,Hash,Eq,PartialEq)]
struct Location{
	x_axis:usize,
	y_axis:usize,
}



fn main(){
let xmas_input = std::fs::read_to_string("day_04.txt".to_string()).expect("no xmas for you");
let xmas_count = find_xmas(&xmas_input);
println!("Solution part one = {}",xmas_count);
}

fn find_xmas(input:&String)->u32{
let mut total_xmas_count = 0;
let line_l:usize = input.lines().next().expect("yo wtf").chars().count();
let len_l:usize = line_l -3;
for line in input.lines(){
	for (current_position,_) in line[..len_l].chars().enumerate(){
		let comp_me= &line[current_position..current_position+4];
	if *comp_me == *XMAS || *comp_me == *DRUNK_XMAS {
		total_xmas_count +=1;
		}
	}
}


let mut map:HashMap<Location,char> = HashMap::new();

	
	for (current_dept,line) in input.lines().enumerate(){

		for (current_car,c) in line.chars().enumerate(){

			let location = Location{
				x_axis:current_dept,
				y_axis:current_car,
	};
			map.insert(location,c);

		}
	}

	let mut outer_y_position = 0;
	let amount_of_lines = input.chars().filter(|c| *c == '\n').count();	
	for _ in 0..amount_of_lines -3{
	let mut x_position= 0;
	let y_position = 0;
	for _ in 0..len_l{
			let zeroth = Location{
				x_axis:x_position,
				y_axis:y_position  + outer_y_position,
			};
				let mut value_to_compare = String::new();
				let v_to_push = map.get(&zeroth);
				value_to_compare.push(*v_to_push.expect("jojo reference?"));

			let zeroth = Location{
				x_axis:x_position + 1,
				y_axis:y_position + 1 + outer_y_position,
			};
				let v_to_push = map.get(&zeroth);
				value_to_compare.push(*v_to_push.expect("jojo reference?"));
	
			let zeroth = Location{
				x_axis:x_position + 2,
				y_axis:y_position + 2 + outer_y_position,
			};
				let v_to_push = map.get(&zeroth);
				value_to_compare.push(*v_to_push.expect("jojo reference?"));

			let zeroth = Location{
				x_axis:x_position + 3,
				y_axis:y_position + 3 + outer_y_position,
			};
				let v_to_push = map.get(&zeroth);
				value_to_compare.push(*v_to_push.expect("jojo reference?"));

			if value_to_compare == XMAS || value_to_compare ==  DRUNK_XMAS {
				total_xmas_count +=1;
			}
			x_position +=1;	
			}
	outer_y_position +=1;
	}
total_xmas_count
}
