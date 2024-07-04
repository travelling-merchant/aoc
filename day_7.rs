use std::collections::HashMap;
use std::io::Error;
fn main(){
	let file_content = vec!["123 -> x","456 -> y","x AND y -> d","x OR y -> e", "x LSHIFT 2 -> f","y RSHIFT 2 -> g"," NOT x -> h","NOT y -> i"];


	let wire_values: HashMap<&str,u16> = HashMap::new();
	let t = insert_basic_wire_assignment(wire_values,file_content);

	println!("sdf2sdfsf{:#?}",t);
}

fn insert_basic_wire_assignment<'a>(wire_map:HashMap<&'a str,u16>,file_input:Vec<&'a str>)->Result<HashMap<&'a str,u16>,Error>{
	let mut wire_values: HashMap<&str,u16> = wire_map;
	let input:Vec<&str> = file_input;
	for entry in input{
		let first_split = entry.split_whitespace().next().expect("FML");
		let last_split = entry.split_whitespace().last().expect("FML");
	
		let is_it_a_number = first_split.parse::<u16>();
		match is_it_a_number{
			Ok(_ok)=>{
				wire_values.insert(last_split,is_it_a_number.expect("NOSLEEP BUT 3 AM"));
			}
			Err(_)=>println!("Stop THERE THIS NO NUMBER HUH????????"),
		}
					
	}
Ok(wire_values)
}
