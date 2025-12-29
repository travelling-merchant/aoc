use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Cordinates {
    pub x_pos: i32,
    pub y_pos: i32,
}
fn main() {
    let file =
        std::fs::read_to_string("day_04.txt".to_string()).expect("they are losing to the sleep");
    let (result_one, result_two) = count_me(file);
    println!("Solution one = {}", result_one);
    println!("Solution two = {}", result_two);
}
fn count_me(input: String) -> (u32, u32) {
    let mut result_one = 0;
    let mut result_two = 0;

    let mut our_map: HashMap<Cordinates, bool> = HashMap::new();
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            let cord = Cordinates {
                x_pos: char_index as i32,
                y_pos: line_index as i32,
            };
            let mut is_paper = false;
            if c == '@' {
                is_paper = true;
            }
            our_map.insert(cord, is_paper);
        }
    }

    let mut is_first = true;
    loop {
        let cords_to_change: Vec<Cordinates> = our_map
            .iter()
            .filter(|v| *v.1)
            .filter(|entry| repeat_me(entry, &our_map))
            .map(|(cord, _)| cord.clone())
            .collect();

        if cords_to_change.is_empty() {
            break;
        }

        for cord in cords_to_change {
            our_map.entry(cord).and_modify(|v| *v = false);
            if is_first {
                result_one += 1;
            }
            result_two += 1;
        }
        is_first = false;
    }

    (result_one, result_two)
}

fn repeat_me(entry: &(&Cordinates, &bool), map: &HashMap<Cordinates, bool>) -> bool {
    let mut coords = vec![];

    for i in entry.0.x_pos - 1..=entry.0.x_pos + 1 {
        for j in entry.0.y_pos - 1..=entry.0.y_pos + 1 {
            let friendly_neighbour = Cordinates {
                x_pos: i as i32,
                y_pos: j as i32,
            };
            coords.push(friendly_neighbour);
        }
    }
    let cord_count = coords
        .iter()
        .map(|cord| map.get(cord))
        .filter(|v| *v == Some(&true))
        .count();
    if cord_count < 5 {
        return true;
    }
    false
}
