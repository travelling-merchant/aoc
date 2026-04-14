fn main() {
    let f = std::fs::read_to_string("day_02.txt").expect("⋆˚✿˖°");
    let mut f: Vec<_> = f
        .lines()
        .map(|n| {
            n.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut result_one: usize = 0;
    f.iter().for_each(|n| {
        result_one += n
            .iter()
            .max()
            .expect("⋆.˚ ☾⭒.˚")
            .abs_diff(*n.iter().min().expect("⋆.˚ ☾⭒.˚"))
    });
    f.iter_mut().for_each(|n| n.sort());
    let mut result_two = 0;
    for entry in f.iter() {
        let max = entry.len() - 1;
        for num in 0..max {
            for i in 0..=max {
                if entry[max - num] % entry[i] == 0 && entry[max - num] != entry[i] {
                    result_two += entry[max - num] / entry[i];
                }
            }
        }
    }
    println!("Solution part one = {:#?}", result_one);
    println!("Solution part two = {:#?}", result_two);
}
