use std::collections::HashMap;
use std::io::Error;

fn main(){
	let file_content = vec!["123 -> x","456 -> y","x AND y -> d","x OR y -> e", "x LSHIFT 2 -> f","y RSHIFT 2 -> g"," NOT x -> h","NOT y -> i"];


	let wire_values: HashMap<&str,i32> = HashMap::new();
	let map_with_keys = fill_map_with_keys_wow(wire_values.clone(),file_content.clone());
	let update_map_with_basic_assignment = insert_basic_wire_assignment(map_with_keys.clone(),file_content.clone());
	//add droped of valready filled in values in vec
	let _work_in_prog = actually_fill_map(map_with_keys.clone(),file_content.clone());

	println!("map after value assignemtn{:#?} default map {:#?}",update_map_with_basic_assignment,map_with_keys);
	println!("map after value assignemtn{:#?} default map {:#?}",update_map_with_basic_assignment,map_with_keys);
}
fn actually_fill_map<'a>(map:HashMap<&'a str ,i32>, file_input:Vec<&'a str>)->HashMap<&'a str,i32>{
	let _vec_to_be_destroyed = file_input.clone();
	let mut temp_map = map;

	for entry in file_input{
			if entry.contains("->") {
				let value = entry.split_whitespace().last().expect("yes, of course... ");		
				let value_to_be_copied = entry.split_whitespace().next().expect("why not working");		
				let default_value = -1;
				if *temp_map.get(value).unwrap() != default_value{
					let new_value = temp_map.get(value).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_copied,*new_value);
					}
				}
			else{ println!("something went wrong");}
			
		};
		temp_map	
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
