const TEST_INPUT: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: i8,
}

fn main() {
    let test_ingredients = construct_ingredients(TEST_INPUT);
	let test_scroe = aquire_cookie_score(test_ingredients);
}


fn aquire_cookie_score(input:Vec<Ingredient>)->i64{
		
}

fn construct_ingredients(input: &str) -> Vec<Ingredient> {
    let mut all_ingredients: Vec<Ingredient> = Vec::new();
    for line in input.lines() {
        let temp_vec: Vec<char> = line
            .chars()
            .filter(|charu| charu != &',' && charu != &':')
            .collect();
        let temp_vec: String = temp_vec.iter().collect();
        let temp_vec_too: Vec<_> = temp_vec.split_whitespace().collect();

        let ingredient = Ingredient {
            name: temp_vec_too[0].to_string(),
            capacity: temp_vec_too[2].parse().expect("can carry things"),
            durability: temp_vec_too[4].parse().expect("can withstand things"),
            flavor: temp_vec_too[6].parse().expect("has no flavour"),
            texture: temp_vec_too[8].parse().expect("can you touch it"),
            calories: temp_vec_too[10].parse().expect("going to gain weight?"),
        };

        println!("{:?}", ingredient);
        all_ingredients.push(ingredient);
    }
    println!("{}", input);
    all_ingredients
}
