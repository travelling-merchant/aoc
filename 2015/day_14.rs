#[derive(Clone, PartialEq)]
struct Rendeer {
    name: String,
    km: u16,
    flight_time: u16,
    rest_time: u16,
    points: u16,
}
const RACE_TIME: u16 = 2503;
fn main() {
    let input = std::fs::read_to_string("day_14.txt".to_string()).expect("is it that simple?");
    let santas_things = santas_stuff(input);
    let (result_one, result_two) = do_things(santas_things);

    println!("solution part one = {}", result_one);
    println!("solution part two = {}", result_two);
}

fn do_things(mut rendeers: Vec<Rendeer>) -> (u16, u16) {
    let mut distance: u16 = 0;
    let mut rendeer_collection: Vec<Rendeer> = Vec::new();
    for current_time in 1..RACE_TIME {
        for deer in &mut rendeers {
            let time = deer.flight_time + deer.rest_time;
            let travel_cycles = current_time / time;
            let remainder = current_time % time;
            let whats_this = deer.flight_time * deer.km * travel_cycles as u16;
            let deer_distance = std::cmp::min(remainder, deer.flight_time) * deer.km;
            let result = deer_distance + whats_this;

            if result > distance {
                rendeer_collection.clear();
                rendeer_collection.push(deer.clone());
                distance = result;
            } else if result == distance {
                rendeer_collection.push(deer.clone());
            }
        }
            
		rendeers.iter_mut().filter(|deer| rendeer_collection.contains(deer)).for_each(|f| f.points += 1);
    }
    let highest_points: u16 = rendeers
        .iter()
        .map(|deer| deer.points)
        .max()
        .expect("how efficent are expectations?");

    (distance, highest_points)
}

fn santas_stuff(input: String) -> Vec<Rendeer> {
    let mut santas_collection: Vec<Rendeer> = Vec::new();
    for line in input.lines() {
        let entry: Vec<_> = line.split_whitespace().collect();
        let current_animal = Rendeer {
            name: entry[0].to_string(),
            km: entry[3].parse().expect("you wanna get dissapointed"),
            flight_time: entry[6].parse().expect("you too"),
            rest_time: entry[13].parse().expect("ah fuck it"),
            points: 0,
        };
        santas_collection.push(current_animal);
    }
    santas_collection
}
