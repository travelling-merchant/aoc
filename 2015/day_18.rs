use std::collections::HashMap;
use std::convert::TryInto;
const LIFE: &str = "day_18.txt";

fn main() {
    println!("calculating..");
    let mut the_light = create_lights();
    the_light = create_life(the_light);
    let the_dark = the_light.clone();
    let mut is_life_or_not: bool = false;

    let part_one_solution = to_be_or_not_to_be(the_light, is_life_or_not);
    println!("Solution part one = {}", part_one_solution);
    println!("please stay tuned the machine is still running");

    is_life_or_not = true;
    let part_two_solution = to_be_or_not_to_be(the_dark, is_life_or_not);

    println!("Solution part two = {}", part_two_solution);
    println!("calculation complete");
}

fn to_be_or_not_to_be(mut current_state: HashMap<(u16, u16), u8>, conway_or_not: bool) -> u16 {
    let light_on: u8 = 1;
    let light_off: u8 = 0;
    for _ in 0..100 {
        let mut new_state: HashMap<(u16, u16), u8> = create_lights();

        for line in 1..=100 {
            for position in 1..=100 {
                let mut all_neighbors: [u8; 8] = [0; 8];

                let current_pos = current_state.get(&(position as u16, line as u16));
                let next_pos = current_state.get(&((position + 1) as u16, line as u16));
                all_neighbors[0] = *next_pos.unwrap();
                let pev_pos = current_state.get(&((position - 1) as u16, line as u16));
                all_neighbors[1] = *pev_pos.unwrap();

                let mut line_above_pos = line;
                line_above_pos -= 1;

                let mut line_below_pos = line;
                line_below_pos += 1;

                let middle_above = current_state.get(&(position as u16, line_above_pos as u16));
                let next_above = current_state.get(&((position + 1) as u16, line_above_pos as u16));
                let prev_above = current_state.get(&((position - 1) as u16, line_above_pos as u16));
                all_neighbors[2] = *middle_above.unwrap();
                all_neighbors[3] = *next_above.unwrap();
                all_neighbors[4] = *prev_above.unwrap();

                let middle_bellow = current_state.get(&(position as u16, line_below_pos as u16));
                let next_bellow =
                    current_state.get(&((position + 1) as u16, line_below_pos as u16));
                let prev_bellow =
                    current_state.get(&((position - 1) as u16, line_below_pos as u16));
                all_neighbors[5] = *middle_bellow.unwrap();
                all_neighbors[6] = *next_bellow.unwrap();
                all_neighbors[7] = *prev_bellow.unwrap();
                let dead_or_alive: u8 = all_neighbors.iter().sum();
                if current_pos == Some(&light_off) {
                    if dead_or_alive == 3 {
                        new_state.insert((position, line), light_on);
                    } else {
                        new_state.insert((position, line), light_off);
                    }
                } else if current_pos == Some(&light_on) {
                    if dead_or_alive == 2 || dead_or_alive == 3 {
                        new_state.insert((position, line), light_on);
                    } else {
                        new_state.insert((position, line), light_off);
                    }
                }
                if conway_or_not {
                    new_state.insert((1 as u16, 1 as u16), light_on);
                    new_state.insert((1 as u16, 100 as u16), light_on);
                    new_state.insert((100 as u16, 1 as u16), light_on);
                    new_state.insert((100 as u16, 100 as u16), light_on);
                }

            }
        }
        current_state = new_state;
    }
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

    }
    the_light
}

fn create_lights() -> HashMap<(u16, u16), u8> {
    let grid_size = 10504;
    let mut nice_gid: HashMap<(u16, u16), u8> = HashMap::new();
    let mut x_index: u16 = 0;
    let mut y_index: u16 = 0;

    for _ in 0..=grid_size {
        if x_index < 102 {
            nice_gid.insert((x_index, y_index), 0);
            x_index += 1;
        } else {
            x_index = 0;
            y_index += 1;
        }
        if y_index == 101 {
        }
    }
    nice_gid
}
