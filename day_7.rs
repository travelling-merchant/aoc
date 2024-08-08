use std::collections::HashMap;
use std::fs;
fn main() {
    let (map_step_one, input) = retrive_instructions();
    let result_two = proccess_instructions(map_step_one, input);
    println!("The solution to part one is {}", result_two.get("a").expect("this WAS SUPPOSED TO BE THE FUCKING SOLUTION"));
}
fn retrive_instructions() -> (HashMap<String, u16>, Vec<String>) {
    let mut map: HashMap<String, u16> = HashMap::new();
    let file = fs::read_to_string("day_7.txt".to_string()).expect("what is a file");
    let _test_string = "x -> m \n 123 -> x \n 456 -> y \n x AND y -> d \n x OR y -> e \n x LSHIFT 2 -> f \n y RSHIFT 2 -> g \n NOT x -> h \n NOT y -> i";
    let mut v: Vec<String> = Vec::new();
    for line in file.lines() {
        let first_element = line.split_whitespace().nth(0).expect("slowly returning");
        let last_element = line
            .split_whitespace()
            .last()
            .expect("who came up with next on first element");
        if line.split_whitespace().collect::<Vec<_>>().len() == 3
            && first_element.chars().all(char::is_numeric) == true
        {
            map.insert(
                last_element.to_string(),
                first_element
                    .parse()
                    .expect("wait I checked this number wtf"),
            );
        } else {
            v.push(line.to_string());
        }
    }
    (map, v)
}
fn proccess_instructions(
    mut map: HashMap<String, u16>,
    mut inst: Vec<String>,
) -> HashMap<String, u16> {

    while inst.len() > 0 {
        let mut to_remove: Vec<usize> = Vec::new();
        for (i, entry) in inst.iter().enumerate() {
            let first = entry.split_whitespace().nth(0).expect("salad?");
            let second = entry.split_whitespace().nth(1).expect("no salad?");

            let third = entry.split_whitespace().nth(2).expect("jelly?");

            if first == "NOT" {
                if map.contains_key(second) {
                    let fourth = entry.split_whitespace().nth(3).expect("jelly?");
                    let result = map.get(second).expect("dont fall asleep");
                    let result = !result;
                    map.insert(fourth.to_string(), result);
                    to_remove.push(i);
                }
            } else if second == "LSHIFT" {
                if map.contains_key(first) {
                    let fifth = entry
                        .split_whitespace()
                        .nth(4)
                        .expect("this can be to long so I cannot get this value for all?");
                    let result = map.get(first).expect("decision fatique");
                    let shift_val: u8 = third.parse().expect("how to solve something");
                    let result = result << shift_val;
                    map.insert(fifth.to_string(), result);
                    to_remove.push(i);
                }
            } else if second == "RSHIFT" {
                if map.contains_key(first) {
                    let fifth = entry
                        .split_whitespace()
                        .nth(4)
                        .expect("this can be to long so I cannot get this value for all?");
                    let result = map.get(first).expect("decision fatique");
                    let shift_val: u16 = third.parse().expect("how to solve something");
                    let result = result >> shift_val;
                    map.insert(fifth.to_string(), result);
                    to_remove.push(i);
                }
            } else if second == "OR" {
                if map.contains_key(first) && map.contains_key(third) {
                    let fifth = entry
                        .split_whitespace()
                        .nth(4)
                        .expect("this can be to long so I cannot get this value for all?");
                    let value_one: u16 = *map.get(first).expect("decision fatique");
                    let value_two: u16 = *map.get(third).expect("how to badly");
                    let result = value_two | value_one;
                    map.insert(fifth.to_string(), result);
                    to_remove.push(i);
                }
            } else if second == "AND" {
                let fifth = entry
                    .split_whitespace()
                    .nth(4)
                    .expect("this can be to long so I cannot get this value for all?");
                if map.contains_key(third) && first == "1" {
                    let value_one: u16 = 1;
                    let value_two: u16 = *map.get(third).expect("how to badly");
                    let result = value_two & value_one;
                    map.insert(fifth.to_string(), result);
                    to_remove.push(i);
                } else if map.contains_key(first) && map.contains_key(third) {
                    let value_one: u16 = *map.get(first).expect("decision fatique");
                    let value_two: u16 = *map.get(third).expect("how to badly");
                    let result = value_two & value_one;
                    map.insert(fifth.to_string(), result);
                    to_remove.push(i);
                }
            } else if second == "->" {
                if map.contains_key(first) {
                    let result = map.get(first).expect("purple light is best");
                    map.insert(third.to_string(), *result);
                    to_remove.push(i);
                }
            }
        }
        let mut inst2 = Vec::new();
        for (i, e) in inst.iter().enumerate() {
            if !to_remove.contains(&i) {
                inst2.push(e.to_string());
            }
        }
        inst = inst2;
    }
    map
}
