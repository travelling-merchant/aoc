use std::collections::HashSet;
const FILE_NAME: &str = "day_19.txt";
#[derive(Debug)]
struct TurnDogsIntoCats {
    origin: String,
    new_owo: String,
}
fn main() {
    let (aahhhh, whaaa) = i_wanna_see_the_horror();
    let solution_one = iterate_trough(aahhhh, whaaa);
    println!("solution part one =  {}", solution_one);
}
fn i_wanna_see_the_horror() -> (Vec<TurnDogsIntoCats>, String) {
    let mut fuck_you: Vec<TurnDogsIntoCats> = Vec::new();
    let mut target_victim = String::new();
    let please_scare_me_lol = std::fs::read_to_string(FILE_NAME).expect("Get your shit together");
    for scare in please_scare_me_lol.lines() {
        if scare.contains("=>") {
            let splits_haha: Vec<_> = scare.split("=>").collect();
            let og: String = splits_haha
                .first()
                .expect("if you see this, then it can't be helped")
                .trim()
                .to_string();
            let ng: String = splits_haha
                .last()
                .expect("if you see this, then it can't be helped")
                .trim()
                .to_string();
            let mutate = TurnDogsIntoCats {
                origin: og,
                new_owo: ng,
            };
            fuck_you.push(mutate);
        } else {
            target_victim = scare.to_string();
        }
    }
    (fuck_you, target_victim)
}

fn iterate_trough(masters_instruction: Vec<TurnDogsIntoCats>, test_subject: String) -> usize {
    let mut all_molecules: HashSet<String> = HashSet::new();
    for instruction in masters_instruction.iter() {
        let instruct_len = instruction.origin.len();
        for (indexu, cut) in test_subject.as_bytes().windows(instruct_len).enumerate() {
            if cut == instruction.origin.as_bytes() {
                let original_subject = test_subject.clone();
                let first_sub: String = original_subject[0..indexu].to_string();
                let new_part = instruction.new_owo.clone();
                let last_sub = original_subject[indexu + instruct_len..].to_string();
                let chimera = format!("{}{}{}", first_sub, new_part, last_sub);
                all_molecules.insert(chimera);
            }
        }
    }
    all_molecules.len()
}
