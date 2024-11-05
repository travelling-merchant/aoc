use std::fs;
fn main() {
    let locations = create_the_list();
    //println!("{:?}",locations);
    let m = permutations(locations);
    //println!("{:#?}", &m);
    println!("{:#?}", &m.len());
    let journey_dt = get_journey_data();
    let r_one = calc_tsp(journey_dt, m);
    println!("{:#?}", r_one);
}
#[derive(Debug)]
struct Journey {
    start_loc: String,
    target_loc: String,
    dist: u16,
}
fn calc_tsp(journeys: Vec<Journey>, routes: Vec<Vec<String>>) -> u16 {
    let mut total_dist_count = u16::MAX;
    //println!("{:#?}",&journeys);
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
                    }
                }
            }
        }
    }
    total_dist_count
}

fn get_journey_data() -> Vec<Journey> {
    let raw_data = fs::read_to_string("day_9.txt".to_string())
        .expect("If you see this, you fucked up real bad lol");
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
    let mut max = locations.len() - 1;
    let mut all_num = locations.len();
    while max > 0 {
        all_num *= max;
        max -= 1;
    }
    let mut counter = 0;
    all_num -= 1;

    all_combos.push(locations.clone());
    while counter < all_num {
        locations[shift_range..len].rotate_right(1);
        if locations != all_combos[0] {
            all_combos.push(locations.clone());
            counter += 1;
        } else {
            if shift_range > 0 {
                shift_range -= 1;
            }
        }
    }
    all_combos
}
