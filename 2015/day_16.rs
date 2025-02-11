const DATA_FILE_NAME: &str = "day_16.txt";

const BEST_AUNT: Aunt = Aunt {
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

#[derive(PartialEq, Debug, Default)]
pub struct Aunt {
    pub children: u8,
    pub cats: u8,
    pub samoyeds: u8,
    pub pomeranians: u8,
    pub akitas: u8,
    pub vizslas: u8,
    pub goldfish: u8,
    pub trees: u8,
    pub cars: u8,
    pub perfumes: u8,
}

fn main() {
    let suspect_data = collect_suspect_data();
    match suspect_data {
        Ok((real_aount, misscalculated_aunt)) => {
            println!("Solution part one = {}", misscalculated_aunt);
            println!("Solution part two = {}", real_aount);
        }
        Err(error) => {
            println!("...{} ", error);
        }
    }
}

fn collect_suspect_data() -> Result<(usize, usize), String> {
    let mut number_of_aunt_calced_wrong: usize = 0;
    let mut okay_real_aunt_for_real: usize = 0;
    let suspect_data = match std::fs::read_to_string(DATA_FILE_NAME.to_string()) {
        Ok(suspect_data) => suspect_data,
        Err(_err) => {
            return Err(
                "so what did you think would happen without providing a input file".to_string(),
            )
        }
    };
    for (index, line) in suspect_data.lines().enumerate() {
        let mut cut_of_good_aunt: u8 = 0;
        let mut real_aunt_count: i8 = 0;

        let potential_aount = profile_aount(line);
        if potential_aount.children == BEST_AUNT.children {
            cut_of_good_aunt += 1;
            real_aunt_count += 1;
        }
        if potential_aount.children != BEST_AUNT.children {
            real_aunt_count -= 1;
        }
        if potential_aount.samoyeds == BEST_AUNT.samoyeds {
            cut_of_good_aunt += 1;
            real_aunt_count += 1;
        }
        if potential_aount.samoyeds != BEST_AUNT.samoyeds {
            real_aunt_count -= 1;
        }
        if potential_aount.cats == BEST_AUNT.cats {
            cut_of_good_aunt += 1;
        }
        if potential_aount.pomeranians == BEST_AUNT.pomeranians {
            cut_of_good_aunt += 1;
        }
        if potential_aount.goldfish == BEST_AUNT.goldfish {
            cut_of_good_aunt += 1;
        }
        if potential_aount.trees == BEST_AUNT.trees {
            cut_of_good_aunt += 1;
        }
        if potential_aount.cars == BEST_AUNT.cars {
            cut_of_good_aunt += 1;
            real_aunt_count += 1;
        }
        if potential_aount.cars != BEST_AUNT.cars {
            real_aunt_count -= 1;
        }
        if potential_aount.perfumes == BEST_AUNT.perfumes {
            cut_of_good_aunt += 1;
            real_aunt_count += 1;
        }
        if potential_aount.perfumes != BEST_AUNT.perfumes {
            real_aunt_count -= 1;
        }

        if potential_aount.cats > BEST_AUNT.cats {
            real_aunt_count += 1;
        }
        if potential_aount.trees > BEST_AUNT.trees {
            real_aunt_count += 1;
        }
        if potential_aount.pomeranians < BEST_AUNT.pomeranians {
            real_aunt_count += 1;
        }
        if potential_aount.goldfish < BEST_AUNT.goldfish {
            real_aunt_count += 1;
        }

        if cut_of_good_aunt > 2 {
            number_of_aunt_calced_wrong = index + 1;
        }
        if real_aunt_count > 2 {
            okay_real_aunt_for_real = index + 1;
        }
    }
    Ok((okay_real_aunt_for_real, number_of_aunt_calced_wrong))
}

fn profile_aount(line: &str) -> Aunt {
    let mut aunt = Aunt::default();
    let collected_attributes: Vec<_> = line.split_whitespace().collect();
    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "perfumes:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("fuck you")
                .parse()
                .expect("am I going to fix this?");
            aunt.perfumes = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "cars:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("i started motivated ")
                .parse()
                .expect("but then someone types expect");
            aunt.cars = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "trees:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("so why not just ")
                .parse()
                .expect("expect");
            aunt.trees = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "goldfish:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("rather than")
                .parse()
                .expect("hading errors");
            aunt.goldfish = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "vizslas:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("just copy")
                .parse()
                .expect("the code ");
            aunt.vizslas = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "akitas:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("be lazy")
                .parse()
                .expect("and go to sleep ");
            aunt.akitas = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "pomeranians:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("...")
                .parse()
                .expect(".............");
            aunt.pomeranians = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "samoyeds:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("so")
                .parse()
                .expect("what");
            aunt.samoyeds = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "cats:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("writing")
                .parse()
                .expect("bad code");
            aunt.cats = num;
        }
    }

    for (index, entry) in collected_attributes.iter().enumerate() {
        if *entry == "children:" {
            let num: u8 = collected_attributes[index + 1]
                .split(',')
                .next()
                .expect("for")
                .parse()
                .expect("a bad solution");
            aunt.children = num;
        }
    }

    aunt
}
