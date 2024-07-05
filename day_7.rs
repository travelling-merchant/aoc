use std::collections::HashMap;
use std::io::Error;

fn main(){
	let file_content = vec!["123 -> x","456 -> y","x AND y -> d","x OR y -> e", "x LSHIFT 2 -> f","y RSHIFT 2 -> g"," NOT x -> h","NOT y -> i"];


	let wire_values: HashMap<&str,i32> = HashMap::new();
	let map_with_keys = fill_map_with_keys_wow(wire_values.clone(),file_content.clone());
	let update_map_with_basic_assignment = insert_basic_wire_assignment(map_with_keys.clone(),file_content.clone());
	println!("map after value assignemtn{:#?} default map {:#?}",update_map_with_basic_assignment,map_with_keys);
}

// while entry in hasmap that is != -1 still in string copy 
//check if its in string entry
//if yes and A not operator aswell do calc and add to hasmap
//if yes and A rshift operator aswell do calc and add to hasmap
//if yes and A lshift operator aswell do calc and add to hasmap
//if yes and A and operator aswell do calc and add to hasmap
//if yes and A or operator aswell do calc and add to hasmap
//drop that vec entry so while loop can end

// and or as last because the are only symbols

fn fill_map_with_keys_wow<'a>(map:HashMap<&'a str,i32>,file_input:Vec<&'a str>)->HashMap<&'a str,i32>{
	let mut empty_map = map;
	let input = file_input;
	for entry in input{
		let v:Vec<_> = entry.split_whitespace().collect();
			for value in v{
				if value.chars().all(char::is_alphanumeric) && value.chars().all(char::is_lowercase){
					let default_value:i32 = -1;
					empty_map.insert(value,default_value);
					}
			}
	}
	empty_map
}


fn insert_basic_wire_assignment<'a>(wire_map:HashMap<&'a str,i32>,file_input:Vec<&'a str>)->Result<HashMap<&'a str,i32>,Error>{
	let mut wire_values: HashMap<&str,i32> = wire_map;
	let input:Vec<&str> = file_input;
	for entry in input{
		let first_split = entry.split_whitespace().next().expect("FML");
		let last_split = entry.split_whitespace().last().expect("FML");
	
		let is_it_a_number = first_split.parse::<i32>();
		match is_it_a_number{
			Ok(_ok)=>{
				wire_values.insert(last_split,is_it_a_number.expect("NOSLEEP BUT 3 AM"));
			}
			Err(_)=>println!("Stop THERE THIS NO NUMBER HUH????????"),
		}
					
	}
Ok(wire_values)
}
