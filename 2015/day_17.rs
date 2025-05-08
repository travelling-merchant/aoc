use std::convert::TryInto;

const HARD_MODE: [u8; 20] = [
    33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
];
const _EASY_MODE: [u8; 5] = [20, 15, 10, 5, 5];

fn main() {
    let (result_one, result_two) = fml();
    println!("the solution to part one = {}", result_one);
    println!("the solution to part two = {}", result_two);
}

fn fml() -> (u16, u16) {
    let mut combination_vec_uwu: Vec<u8> = Vec::new();
    let mut all_tried_combinations: Vec<Vec<u8>> = Vec::new();
    let _easy_mode_target: u16 = 25;
    let hard_mode_target: u16 = 150;
    let the_void: u8 = 0;

    recursion_scary(
        &mut all_tried_combinations,
        the_void.into(),
        hard_mode_target,
        &mut combination_vec_uwu,
        &HARD_MODE,
    );
    let result: u16 = all_tried_combinations.len().try_into().unwrap();
    let mut smallest: usize = usize::MAX;
    for entry in all_tried_combinations.iter() {
        if entry.len() <= smallest {
            smallest = entry.len();
        }
    }
    let result_two: u16 = all_tried_combinations
        .iter()
        .filter(|combo| combo.len() == smallest)
        .count()
        .try_into()
        .unwrap();
    (result, result_two)
}

fn recursion_scary(
    all_combos: &mut Vec<Vec<u8>>,
    start: usize,
    target_number: u16,
    current_byte_spell: &mut Vec<u8>,
    input: &[u8],
) {
    let summary: u16 = current_byte_spell.iter().map(|&n| n as u16).sum();

    if summary == target_number {
        all_combos.push(current_byte_spell.clone());
        return;
    } else if summary > target_number {
        return;
    }

    for element in start..input.len() {
        current_byte_spell.push(input[element]);
        recursion_scary(
            all_combos,
            element + 1,
            target_number,
            current_byte_spell,
            input,
        );
        current_byte_spell.pop();
    }
}
