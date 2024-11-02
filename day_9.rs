use std::fs;
fn main() {
    let locations = create_the_list();
    //println!("{:?}",locations);
    let m = permutations(locations);
//    println!("{:#?}", &m);
    println!("{:#?}", m.len());
}

fn create_the_list() -> Vec<String> {
    let mut locations: Vec<String> = Vec::new();
    let raw_list = fs::read_to_string("day_9.txt".to_string())
        .expect("what are you doing again, you don't even have a file");
    for line in raw_list.lines() {
        let first_loc = line
            .split_whitespace()
            .next()
            .expect("your data is shit")
            .to_string();
        let second_loc = line
            .split_whitespace()
            .nth(2)
            .expect("but don't worry so is your code")
            .to_string();
        if !locations.contains(&first_loc) {
            locations.push(first_loc);
        }
        if !locations.contains(&second_loc) {
            locations.push(second_loc);
        }
    }
    locations
}
fn permutations(mut locations: Vec<String>) -> Vec<Vec<String>> {
    let mut all_combos: Vec<Vec<String>> = Vec::new();
    let len = locations.len();
    let mut shift_range = len - 2;
	
	// calculate factorial number
    let mut max = locations.len()-1;
	let mut all_num = locations.len();
	while max > 0{
	all_num *=  max;
	max -=1	;
	}
    let mut counter = 0;
	all_num  -=1;

    all_combos.push(locations.clone());
	while counter < all_num {
                locations[shift_range..len].rotate_right(1);
                if locations != all_combos[0] {
                    all_combos.push(locations.clone());
					counter +=1;
                } else {
                    if shift_range > 0 {
                        shift_range -= 1;
                    }
                }
	}
    all_combos
}
/*fn permutations(mut locations: Vec<String>) -> Vec<Vec<String>> {
    let mut all_combos: Vec<Vec<String>> = Vec::new();
    let number_x = locations.len();
    for _ in 0..locations.len() {
        let mut ground_zero = 1;
        let max_combo = 40320;
        let counter = 40320;
        all_combos.push(locations.clone());
            while counter < max_combo{
                for _ in ground_zero..number_x{
                    locations[ground_zero..number_x].rotate_right(1);
                    all_combos.push(locations.clone());
                }
                counter;
            }
        locations.rotate_right(1);
    }
    all_combos
}*/
