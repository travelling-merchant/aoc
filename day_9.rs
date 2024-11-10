use std::collections::HashSet;
use std::fs;
use std::time::Instant;
const FILE_NAME: &str = "day_9.txt";
fn main() {
    let start = Instant::now();
    let locations = create_the_list();
    let m = permutations(locations);
    let journey_dt = get_journey_data();
    let (r_one, r_two) = calc_tsp(journey_dt, m);
    println!("Solution to Part 1 = {:#?}", r_one);
    println!("Solution to Part 2 = {:#?}", r_two);
    println!("Time taken {:#?}", start.elapsed());
}
#[derive(Debug)]
struct Journey {
    start_loc: String,
    target_loc: String,
    dist: u16,
}
fn calc_tsp(journeys: Vec<Journey>, routes: HashSet<Vec<String>>) -> (u16, u16) {
    test_uwu(&routes);
    println!("calculate tsp...");
    let mut total_dist_count = u16::MAX;
    let mut total_distaster = 0;
    for route in routes {
        let mut dist_counter: u16 = 0;
        for (i, loc) in route.iter().enumerate() {
            for travel in &journeys {
                if i < (route.len() - 1) {
                    if *loc == travel.start_loc && route[i + 1] == travel.target_loc
                        || *loc == travel.target_loc && travel.start_loc == route[i + 1]
                    {
                        dist_counter += travel.dist;
                    }
                } else {
                    if total_dist_count > dist_counter {
                        total_dist_count = dist_counter;
                    } else if total_distaster < dist_counter {
                        total_distaster = dist_counter;
                    }
                }
            }
        }
    }
    (total_dist_count, total_distaster)
}

fn get_journey_data() -> Vec<Journey> {
    let raw_data =
        fs::read_to_string(FILE_NAME).expect("If you see this, you fucked up real bad lol");
    let mut sweet_journeys: Vec<Journey> = Vec::new();
    for entry in raw_data.lines() {
        let loc_one = entry
            .split_whitespace()
            .nth(0)
            .expect("i have some many questions");
        let loc_two = entry.split_whitespace().nth(2).expect("so what life?");
        let number: u16 = entry
            .split_whitespace()
            .last()
            .expect("why and how is it? also small numbers only hewe")
            .parse::<u16>()
            .expect("small things are cuter");
        let travel = Journey {
            start_loc: loc_one.to_string(),
            target_loc: loc_two.to_string(),
            dist: number,
        };
        sweet_journeys.push(travel);
    }
    sweet_journeys
}

fn create_the_list() -> Vec<String> {
    let mut locations: Vec<String> = Vec::new();
    let raw_list = fs::read_to_string(FILE_NAME)
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
fn permutations(locations: Vec<String>) -> HashSet<Vec<String>> {
    let mut all_combos = HashSet::new();
    all_combos.insert(locations.clone());
    for i in 0..locations.len() {
        let mut remaning = locations.to_vec();
        remaning.remove(i);

        let sub_permutations = permutations(remaning);
        for mut sub_permutation in sub_permutations {
            sub_permutation.insert(0, locations[i].clone());
            all_combos.insert(sub_permutation);
        }
    }

    all_combos
}

fn test_uwu(permutations: &HashSet<Vec<String>>) -> () {
	// this test is absolutely useless now that I use a hashet, but whatever.....
    let correct_len = permutations.len();
    let unique_checked: HashSet<_> = permutations.iter().collect();
    println!("executing test...");
    let actuall_len = unique_checked.len();
    assert_eq!(
        actuall_len, correct_len,
        "the actuall lenght is {} the correct len is {} the permutation function is fucked lol",
        actuall_len, correct_len
    );
    println!("test successfully executed, yay ");
}
