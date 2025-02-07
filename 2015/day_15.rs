const TEST_INPUT: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

const REAL_INPUT: &str = "Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3
Chocolate: capacity 0, durability 0, flavor 5, texture -1, calories 8
Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8";

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
    let (test_score, calories_core) = aquire_cookie_score(test_ingredients);
    let real_ingredients = construct_ingredients(REAL_INPUT);
    let (real_score, best_cookie) = oh_cookies_are_baked_lol(real_ingredients);
    if test_score != 62842880 || calories_core != 57600000 {
        panic!("why don't you try doing something right for once?");
    }
    println!("Solution part one = {}", real_score);
    println!("Solution part two = {}", best_cookie);
}

fn cook_cookie(attriute: i8, number: u32) -> i64 {
    attriute as i64 * number as i64
}
fn cook_again(a: i64, b: i64, c: i64, d: i64) -> i64 {
    if a < 1 || b < 1 || c < 1 || d < 1 {
        0
    } else {
        a * b * c * d
    }
}

fn oh_cookies_are_baked_lol(input: Vec<Ingredient>) -> (i64, i64) {
    let mut result: i64 = 0;
    let mut litle_calories_result: i64 = 0;
    let magic_number: usize = 4;
    let ingrdients_comb = calc_combinations(magic_number);
    for combo in ingrdients_comb {
        let a = cook_cookie(input[0].capacity, combo[0]);
        let b = cook_cookie(input[0].durability, combo[0]);
        let c = cook_cookie(input[0].flavor, combo[0]);
        let d = cook_cookie(input[0].texture, combo[0]);
        let e = cook_cookie(input[0].calories, combo[0]);

        let aa = cook_cookie(input[1].capacity, combo[1]);
        let bb = cook_cookie(input[1].durability, combo[1]);
        let cc = cook_cookie(input[1].flavor, combo[1]);
        let dd = cook_cookie(input[1].texture, combo[1]);
        let ee = cook_cookie(input[1].calories, combo[1]);

        let aaa = cook_cookie(input[2].capacity, combo[2]);
        let bbb = cook_cookie(input[2].durability, combo[2]);
        let ccc = cook_cookie(input[2].flavor, combo[2]);
        let ddd = cook_cookie(input[2].texture, combo[2]);
        let eee = cook_cookie(input[2].calories, combo[2]);

        let aaaa = cook_cookie(input[3].capacity, combo[3]);
        let bbbb = cook_cookie(input[3].durability, combo[3]);
        let cccc = cook_cookie(input[3].flavor, combo[3]);
        let dddd = cook_cookie(input[3].texture, combo[3]);
        let eeee = cook_cookie(input[3].calories, combo[3]);

        let temp_r: i64 = cook_again(
            a + aa + aaa + aaaa,
            b + bb + bbb + bbbb,
            c + cc + ccc + cccc,
            d + dd + ddd + dddd,
        );
        if temp_r > result {
            result = temp_r;
        }
        let amount_of_weigt_gain: i64 = e + ee + eee + eeee;
        if amount_of_weigt_gain == 500 {
            if temp_r > litle_calories_result {
                litle_calories_result = temp_r;
            }
        }
    }
    (result, litle_calories_result)
}

fn aquire_cookie_score(input: Vec<Ingredient>) -> (i64, i64) {
    let amount_of_ingredients = input.len();
    let ingredient_combinations = calc_combinations(amount_of_ingredients);
    let mut litle_calories_result: i64 = 0;
    let mut result: i64 = 0;
    for combo in ingredient_combinations {
        let r = cook_cookie(input[0].capacity, combo[0]);
        let s = cook_cookie(input[0].durability, combo[0]);
        let t = cook_cookie(input[0].flavor, combo[0]);
        let u = cook_cookie(input[0].texture, combo[0]);
        let v = cook_cookie(input[0].calories, combo[0]);

        let rr = cook_cookie(input[1].capacity, combo[1]);
        let ss = cook_cookie(input[1].durability, combo[1]);
        let tt = cook_cookie(input[1].flavor, combo[1]);
        let uu = cook_cookie(input[1].texture, combo[1]);
        let vv = cook_cookie(input[1].calories, combo[1]);

        let temp_r: i64 = cook_again(r + rr, s + ss, t + tt, u + uu);
        let amount_of_weigt_gain: i64 = vv + v;
        if amount_of_weigt_gain == 500 {
            if temp_r > litle_calories_result {
                litle_calories_result = temp_r;
            }
        }
        if temp_r > 62842880 {
            panic!("fuck fuck fuck")
        }
        if temp_r > result {
            result = temp_r;
        }
    }
    (result, litle_calories_result)
}

fn calc_combinations(amount_of_ingredients: usize) -> Vec<Vec<u32>> {
    let mut combinations = Vec::new();
    match amount_of_ingredients {
        2 => {
            for ingredient_alpha in 0..=100 {
                combinations.push(vec![ingredient_alpha, 100 - ingredient_alpha]);
            }
        }
        4 => {
            for ingredient_alpha in 0..=100 {
                for ingredient_beta in 0..=(100 - ingredient_alpha) {
                    let remaining = 100 - ingredient_alpha - ingredient_beta;
                    for ingredient_gamma in 0..=remaining {
                        combinations.push(vec![
                            ingredient_alpha,
                            ingredient_beta,
                            ingredient_gamma,
                            remaining - ingredient_gamma,
                        ]);
                    }
                }
            }
        }
        _ => panic!("learn recursion bitch!"),
    }
    combinations
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

        all_ingredients.push(ingredient);
    }
    all_ingredients
}
