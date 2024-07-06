use std::collections::HashMap;
use std::io::Error;

fn main(){
	let file_content = vec!["123 -> x","456 -> y","x AND y -> d","x OR y -> e", "x LSHIFT 2 -> f","y RSHIFT 2 -> g"," NOT x -> h","NOT y -> i"];


	let wire_values: HashMap<&str,i32> = HashMap::new();
	let map_with_keys = fill_map_with_keys_wow(wire_values.clone(),file_content.clone());
	let update_map_with_basic_assignment = insert_basic_wire_assignment(map_with_keys.clone(),file_content.clone());
	
	let _updated_vec = remove_already_filled_of_vec_because_me_stupid(&update_map_with_basic_assignment.expect(""),file_content.clone());
	let work_in_prog = actually_fill_map(map_with_keys.clone(),file_content.clone());
	println!("full map {:#?}",work_in_prog);
}

fn remove_already_filled_of_vec_because_me_stupid<'a>(map:&HashMap<&'a str ,i32>,file_input:Vec<&'a str>)->Vec<&'a str>{
	let mut fixed_vec = file_input.clone();
	let map_to_read = &map;
	let default_value:i32 = -1;
	let mut no_more_values = 1;	
	while no_more_values > 0{
	
	for (i,entry) in fixed_vec.clone().iter().enumerate(){
		let compare_to = entry.split_whitespace().last();	
		let map_val:i32 = *map_to_read.get(compare_to.expect("LALALALA")).unwrap();
		if map_val != default_value{
		no_more_values +=1;
		fixed_vec.remove(i);
		}
	}
no_more_values -=1;
}
println!("fixed vector {:#?}",fixed_vec);
fixed_vec
} 
fn actually_fill_map<'a>(map:HashMap<&'a str ,i32>, file_input:Vec<&'a str>)->HashMap<&'a str,i32>{
	let mut vec_to_be_destroyed = file_input.clone();
	let mut temp_map = map;
	while vec_to_be_destroyed.is_empty() == false{  

	let default_value = -1;
			for (i,entry) in vec_to_be_destroyed.clone().iter().enumerate(){	
			if entry.contains("->") {
				let value = entry.split_whitespace().last().expect("yes, of course... ");		
				let value_to_be_copied = entry.split_whitespace().next().expect("why not working");		
				if *temp_map.get(value).unwrap() != default_value{
					let new_value = temp_map.get(value).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_copied,*new_value);
					vec_to_be_destroyed.remove(i);
					}
				}
				else if entry.contains("NOT"){
				
				let value_to_be_assigned = entry.split_whitespace().nth(3).expect("yes, of course... ");		
				let value_to_be_copied = entry.split_whitespace().nth(1).expect("why not working");		
				
				if *temp_map.get(value_to_be_copied).unwrap() != default_value{
					let new_value = temp_map.get(value_to_be_assigned).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_copied,!new_value);
					vec_to_be_destroyed.remove(i);
					}
				}
				else if entry.contains("RSHIFT"){
				
				let value_to_be_shifted= entry.split_whitespace().nth(0).expect("yes, of course... ");		
				let value_to_be_number:i32= entry.split_whitespace().nth(2).expect("yes, of course... ").parse().expect("THISNO NUMBER WTF?");		
				let value_to_be_assigned= entry.split_whitespace().nth(4).expect("yes, of course... ");		
				
				if *temp_map.get(value_to_be_shifted).unwrap() != default_value{
					let new_value_too = temp_map.get(value_to_be_shifted).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_assigned,value_to_be_number >> *new_value_too);
					vec_to_be_destroyed.remove(i);
					}
				}
				else if entry.contains("LSHIFT"){
				
				let value_to_be_shifted= entry.split_whitespace().nth(0).expect("yes, of course... ");		
				let value_to_be_number:i32= entry.split_whitespace().nth(2).expect("yes, of course... ").parse().expect("THISNO NUMBER WTF?");		
				let value_to_be_assigned= entry.split_whitespace().nth(4).expect("yes, of course... ");		
				
				if *temp_map.get(value_to_be_shifted).unwrap() != default_value{
					let new_value_too = temp_map.get(value_to_be_shifted).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_assigned,value_to_be_number << *new_value_too);
					vec_to_be_destroyed.remove(i);
					}
				}
				else if entry.contains("AND"){
				
				let value_to_be_added = entry.split_whitespace().nth(0).expect("yes, of course... ");		
				let value_to_be_added_too = entry.split_whitespace().nth(2).expect("why not working");		
				let value_to_be_result = entry.split_whitespace().nth(4).expect("why not working");		
				
				if *temp_map.get(value_to_be_added).unwrap() != default_value && *temp_map.get(value_to_be_added_too).unwrap() != default_value{
					let new_value = temp_map.get(value_to_be_added).expect("why do i need so much sleep");	
					let new_value_too = temp_map.get(value_to_be_added_too).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_result,new_value & new_value_too);
					vec_to_be_destroyed.remove(i);
					}
				}
				else if entry.contains("OR"){
				
				let value_to_be_added = entry.split_whitespace().nth(0).expect("yes, of course... ");		
				let value_to_be_added_too = entry.split_whitespace().nth(2).expect("why not working");		
				let value_to_be_result = entry.split_whitespace().nth(4).expect("why not working");		
				
				if *temp_map.get(value_to_be_added).unwrap() != default_value && *temp_map.get(value_to_be_added_too).unwrap() != default_value{
					let new_value = temp_map.get(value_to_be_added).expect("why do i need so much sleep");	
					let new_value_too = temp_map.get(value_to_be_added_too).expect("why do i need so much sleep");	
					temp_map.insert(value_to_be_result,new_value | new_value_too);
					vec_to_be_destroyed.remove(i);
					}
				}
				else{
				 println!("CRY ABOUT IT ");
				}
			
		};
	}
		println!("hsdfafsfsfsdf {:#?}",vec_to_be_destroyed);
		temp_map	
}

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
