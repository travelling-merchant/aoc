use std::collections::HashMap;
use std::fs;
use std::io::Error;
fn main() {
    // input for testing
    /*let file_content = vec![
        "666 -> ä",
        "777 -> ö",
        "123 -> x",
        "456 -> y",
        "x AND y -> d",
        "g -> ü",
        "x OR y -> e",
        "x LSHIFT 2 -> f",
        "y RSHIFT 2 -> g",
        "NOT x -> h",
        "NOT y -> i",
    ];*/
    let mut file_content:Vec<&str> = Vec::new();
    let f = fs::read_to_string("day_7.txt").expect("sad, you don't have anyone to gift a file to you");
    for line in f.lines()
    {
    file_content.push(line);
    }

    let wire_values: HashMap<&str, i32> = HashMap::new();
    let map_with_keys = fill_map_with_keys_wow(wire_values.clone(), file_content.clone());
    let update_map_with_basic_assignment =
        insert_basic_wire_assignment(map_with_keys.clone(), file_content.clone());

    let (updated_vec, mid_map) = remove_already_filled_of_vec_because_me_stupid(
        update_map_with_basic_assignment.expect(""),
        file_content.clone(),
    );
    let complete = actually_fill_map(mid_map, updated_vec);
    println!("full map {:#?}", complete);
}

fn remove_already_filled_of_vec_because_me_stupid<'a>(
    map: HashMap<&'a str, i32>,
    file_input: Vec<&'a str>,
) -> (Vec<&'a str>, HashMap<&'a str, i32>) {
    let fixed_vec = file_input.clone();
    let map_to_read = map;
    let mut list_with_indexes_to_remove: Vec<usize> = Vec::new();
    for (i, entry) in fixed_vec.iter().enumerate() {
        let first_split = entry.split_whitespace().nth(0);
        if first_split.expect("").chars().next().unwrap().is_numeric() {
            list_with_indexes_to_remove.push(i);
        }
    }
    let mut retained_vec = Vec::new();
    for (i, despair) in fixed_vec.iter().enumerate() {
        if !list_with_indexes_to_remove.iter().any(|hope| *hope == i) {
            retained_vec.push(despair.clone());
        }
    }
    (retained_vec, map_to_read)
}

// turns out im stupid i cant delete 0 here aswell since when its not yet assignable it needs to stay in the vec so I need to do something similar like on the other one.

fn actually_fill_map<'a>(
    map: HashMap<&'a str, i32>,
    file_input: Vec<&'a str>,
) -> HashMap<&'a str, i32> {
    let mut vec_to_be_destroyed = file_input.clone();
    let mut temp_map: HashMap<&str, i32> = map;
    println!("MAP IN Fill function before anything {:#?}", temp_map);
    let default_value = -1;
    let mut no_more_values = 1;
    while no_more_values > 0 {
        println!("current state of vec{:#?}", vec_to_be_destroyed);
        let mut index_counter = 0;
        for (i, entry) in vec_to_be_destroyed.clone().iter().enumerate() {
            println!("current iteraction value is {i}");

            println!("current state of vec{:#?}", vec_to_be_destroyed);
            if entry.contains("NOT") {
                let value_to_be_assigned =
                    entry.split_whitespace().nth(3).expect("yes, of course... ");
                let value_to_be_copied = entry.split_whitespace().nth(1).expect("why not working");

                if *temp_map.get(value_to_be_copied).unwrap() != default_value {
                    let new_value = temp_map
                        .get(value_to_be_copied)
                        .expect("why do i need so much sleep");
                    let new_value_after_calc: u16 = !new_value.clone().to_owned() as u16;
                    //println!("new value not {new_value_after_calc}");
                    temp_map.insert(value_to_be_assigned, new_value_after_calc as i32);
                    no_more_values += 1;
                    //println!("NOT IS EXECTUED LINE BEFORE VEC INDEX DROP");
                    vec_to_be_destroyed.remove(index_counter);
                    //println!("NOT IS EXECTUED LINE AFTER VEC INDEX DROP");
                    //println!("MAP bfter NOT{:#?}",temp_map);
                }
            } else if entry.contains("RSHIFT") {
                let value_to_be_shifted =
                    entry.split_whitespace().nth(0).expect("yes, of course... ");
                let value_to_be_number: i32 = entry
                    .split_whitespace()
                    .nth(2)
                    .expect("yes, of course... ")
                    .parse()
                    .expect("THISNO NUMBER WTF?");
                let value_to_be_assigned =
                    entry.split_whitespace().nth(4).expect("yes, of course... ");

                if *temp_map.get(value_to_be_shifted).unwrap() != default_value {
                    let new_value_too = temp_map
                        .get(value_to_be_shifted)
                        .expect("why do i need so much sleep");
                    temp_map.insert(value_to_be_assigned, *new_value_too >> value_to_be_number);
                    no_more_values += 1;
                    //println!("RSHIFT IS EXECTUED LINE BEFORE VEC INDEX DROP");
                    vec_to_be_destroyed.remove(index_counter);
                    //println!("RSHIFT IS EXECTUED LINE AFTER VEC INDEX DROP");
                }
            } else if entry.contains("LSHIFT") {
                let value_to_be_shifted =
                    entry.split_whitespace().nth(0).expect("yes, of course... ");
                let value_to_be_number: i32 = entry
                    .split_whitespace()
                    .nth(2)
                    .expect("yes, of course... ")
                    .parse()
                    .expect("THISNO NUMBER WTF?");
                let value_to_be_assigned =
                    entry.split_whitespace().nth(4).expect("yes, of course... ");

                if *temp_map.get(value_to_be_shifted).unwrap() != default_value {
                    let new_value_too = temp_map
                        .get(value_to_be_shifted)
                        .expect("why do i need so much sleep");
                    temp_map.insert(value_to_be_assigned, *new_value_too << value_to_be_number);
                    no_more_values += 1;

                    println!("LSHIFT IS EXECTUED LINE BEFORE VEC INDEX DROP");
                    vec_to_be_destroyed.remove(index_counter);
                    //println!("LSHIFT IS EXECTUED LINE AFTER VEC INDEX DROP");
                }
            } else if entry.contains("AND") {
                let value_to_be_added =
                    entry.split_whitespace().nth(0).expect("yes, of course... ");
                let value_to_be_added_too =
                    entry.split_whitespace().nth(2).expect("why not working");
                let value_to_be_result = entry.split_whitespace().nth(4).expect("why not working");

                if *temp_map.get(value_to_be_added).unwrap() != default_value
                    && *temp_map.get(value_to_be_added_too).unwrap() != default_value
                {
                    let new_value = temp_map
                        .get(value_to_be_added)
                        .expect("why do i need so much sleep");
                    let new_value_too = temp_map
                        .get(value_to_be_added_too)
                        .expect("why do i need so much sleep");
                    temp_map.insert(value_to_be_result, new_value & new_value_too);
                    no_more_values += 1;
                    println!("AND IS EXECTUED LINE BEFORE VEC INDEX DROP");
                    vec_to_be_destroyed.remove(index_counter);
                    //println!("AND IS EXECTUED LINE AFTER VEC INDEX DROP");
                }
            } else if entry.contains("OR") {
                let value_to_be_or = entry.split_whitespace().nth(0).expect("yes, of course... ");
                let value_to_be_or_too = entry.split_whitespace().nth(2).expect("why not working");
                let value_to_be_result = entry.split_whitespace().nth(4).expect("why not working");

                if *temp_map.get(value_to_be_or).unwrap() != default_value
                    && *temp_map.get(value_to_be_or).unwrap() != default_value
                {
                    let new_value = temp_map
                        .get(value_to_be_or)
                        .expect("why do i need so much sleep");
                    let new_value_too = temp_map
                        .get(value_to_be_or_too)
                        .expect("why do i need so much sleep");
                    let v_after_calc = new_value | new_value_too;
                    //println!("this should be the value wirth or {v_after_calc}");
                    temp_map.insert(value_to_be_result, v_after_calc);
                    no_more_values += 1;
                    //println!("OR IS EXECTUED LINE BEFORE VEC INDEX DROP");
                    vec_to_be_destroyed.remove(index_counter);
                    //println!("OR IS EXECTUED LINE AFTER VEC INDEX DROP");
                }
            } else if entry.split_whitespace().collect::<Vec<_>>().len() == 3 {
                println!("wowies should be executed once on testin input");
                let value = entry.split_whitespace().last().expect("yes, of course... ");
                let value_to_be_copied = entry.split_whitespace().nth(0).expect("why not working");
                println!("before if");
                if *temp_map.get(value_to_be_copied).unwrap() != default_value {
                    print!("after if");
                    let new_value = temp_map
                        .get(value_to_be_copied)
                        .expect("why do i need so much sleep");
                    temp_map.insert(value, *new_value);
                    no_more_values += 1;
                    println!("ELSE IS EXECTUED LINE BEFORE VEC INDEX DROP ->");
                    vec_to_be_destroyed.remove(index_counter);
                    //println!("ELSE IS EXECTUED LINE AFTER VEC INDEX DROP");
                }else{
			println!("ü g value should leas to this");
                    no_more_values += 1;
			let f = vec_to_be_destroyed[0]; 
                    vec_to_be_destroyed.push(f);
                    vec_to_be_destroyed.remove(0);
                    //println!("ELSE IS EXECTUED LINE AFTER VEC INDEX DROP");
			}
		
            } 
        }

        no_more_values -= 1;
    }
    //println!("hsdfafsfsfsdf {:#?}",vec_to_be_destroyed);
    temp_map
}

fn fill_map_with_keys_wow<'a>(
    map: HashMap<&'a str, i32>,
    file_input: Vec<&'a str>,
) -> HashMap<&'a str, i32> {
    let mut empty_map = map;
    let input = file_input;
    for entry in input {
        let v: Vec<_> = entry.split_whitespace().collect();
        for value in v {
            if value.chars().all(char::is_alphanumeric) && value.chars().all(char::is_lowercase) {
                let default_value: i32 = -1;
                empty_map.insert(value, default_value);
            }
        }
    }
    empty_map
}

fn insert_basic_wire_assignment<'a>(
    wire_map: HashMap<&'a str, i32>,
    file_input: Vec<&'a str>,
) -> Result<HashMap<&'a str, i32>, Error> {
    let mut wire_values: HashMap<&str, i32> = wire_map;
    let input: Vec<&str> = file_input;
    for entry in input {
        let first_split = entry.split_whitespace().next().expect("FML");
        let last_split = entry.split_whitespace().last().expect("FML");

        let is_it_a_number = first_split.parse::<i32>();
        match is_it_a_number {
            Ok(_ok) => {
                wire_values.insert(last_split, is_it_a_number.expect("NOSLEEP BUT 3 AM"));
            }
            Err(_) => print!(""),
        }
    }
    Ok(wire_values)
}
