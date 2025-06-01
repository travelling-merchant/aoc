use std::collections::HashMap;
use std::convert::TryInto;
const LIFE: &str = "day_18.txt";
fn main() {
    let mut the_light = create_lights();
    the_light = create_life(the_light);
	let part_one_solution = to_be_or_not_to_be(the_light);
    //println!("lala {:#?}", the_light);
    println!("Solution part one = {}", part_one_solution);
    //println!("lala {:#?}", file_input);
}

fn to_be_or_not_to_be(mut current_state:HashMap<(u16,u16),u8>)->u16{
    let light_on: u8 = 1;
    let light_off: u8 = 0;
	for _ in 0..100{
	let mut new_state:HashMap<(u16,u16),u8> = create_lights();
	// go over the lines of the grid
	for line in 1..=100{
			
		// go over individual position in line
		for position  in 1..=100{

			let mut all_neighbors:[u8;8] = [0;8];		

			let current_pos = current_state.get(&(position as u16,line as u16));
			let next_pos = current_state.get(&((position+1) as u16,line as u16));
			if next_pos.is_none(){
				//println!("what are you doing again?{}",position+1);
			}	
			all_neighbors[0] = *next_pos.unwrap();
			let pev_pos = current_state.get(&((position-1) as u16,line as u16));
			all_neighbors[1] = *pev_pos.unwrap();

			let mut line_above_pos = line ;
			line_above_pos-= 1;

			let mut line_below_pos = line;
			line_below_pos +=1;

			let middle_above = current_state.get(&(position as u16 ,line_above_pos as u16));
			let next_above = current_state.get(&((position+1) as u16 ,line_above_pos as u16));
			let prev_above= current_state.get(&((position-1) as u16 ,line_above_pos as u16));
			all_neighbors[2] = *middle_above.unwrap();
			all_neighbors[3] = *next_above.unwrap();
			all_neighbors[4] = *prev_above.unwrap();

			
			let middle_bellow= current_state.get(&(position as u16 ,line_below_pos as u16));
			let next_bellow = current_state.get(&((position +1)as u16 ,line_below_pos as u16));
			//println!("wtf is this {}",line_below_pos);
			let prev_bellow = current_state.get(&((position-1) as u16 ,line_below_pos as u16));
			all_neighbors[5] = *middle_bellow.unwrap();
			all_neighbors[6] = *next_bellow.unwrap();
			all_neighbors[7] = *prev_bellow.unwrap();
			let dead_or_alive :u8 = all_neighbors.iter().sum();
			if current_pos == Some(&light_off){
					if dead_or_alive == 3{
						new_state.insert((position,line),light_on);
					}
					else{
						new_state.insert((position,line),light_off);
					}				
			} else if current_pos == Some(&light_on){
					if dead_or_alive == 2 || dead_or_alive == 3{
						new_state.insert((position,line),light_on);
					}
					else{
						new_state.insert((position,line),light_off);
					}
			}
			
			//println!("all 3 positions  {:#?} {:#?} {:#?}",pev_pos,number,next_pos);
			//println!("this is above {:#?}",middle_above);
		}	
	}	
	current_state = new_state;
	}
	println!("current state {:#?}",current_state);
	let values = current_state.values().filter(|v| **v == 1).count();
	values as u16
}

fn create_life(mut the_light: HashMap<(u16, u16), u8>) -> HashMap<(u16, u16), u8> {
    let light_on: u8 = 1;
    let light_off: u8 = 0;
    let may_there_be_light = std::fs::read_to_string(LIFE).expect("havent you forgotten something");

    for (index, line) in may_there_be_light.lines().enumerate() {
        let mut useful_number: u16 = index.try_into().unwrap();
        useful_number += 1;

        for (char_index, charu) in line.chars().enumerate() {
            // for every char if on or off
            // get the line and char index
            // insert state

            let mut real_real_char: u16 = char_index.try_into().unwrap();
            real_real_char += 1;

            if charu == '#' {
                the_light.insert((real_real_char, useful_number), light_on);
            } else if charu == '.' {
                the_light.insert((real_real_char, useful_number), light_off);
            }
        }

        //print!("lenght is {:#?}", line.chars().last());
    }
        //print!("hereee {:#?}", the_light.get(&(101,100)));
    the_light
}

fn create_lights() -> HashMap<(u16, u16), u8> {
	let grid_size = 10504;
    let mut nice_gid: HashMap<(u16, u16), u8> = HashMap::new();
    let mut x_index: u16 = 0;
    let mut y_index: u16 = 0;

    for _ in 0..=grid_size{
        if x_index < 102 {
            nice_gid.insert((x_index, y_index), 0);
            x_index += 1;
        } else {
            x_index = 0;
            y_index += 1;
        }
		if y_index == 101{
		//print!("hahle");
			}
    }
	//for i in 0..=101{
	//		if nice_gid.get(&(i,101)).is_none(){
	//			print!("{}",i);
	//		}
	//}
    nice_gid

}
