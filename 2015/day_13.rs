use std::collections::HashSet;
#[derive(PartialEq,Debug)]
struct HumanEntity{
	name:String,
	relationship_unit:i32,
	target_name:String,
}
fn main(){
	let input = std::fs::read_to_string("day_13.txt".to_string()).expect("still pretty dark outisde");
	let  (people,entities) = nice_api(input);

	let  arrangments = get_the_fucking_permutations(people.clone());
	let result_one = calculate_emotional_distress(arrangments.clone(),&entities);
	println!("Solution part one = {:#?}",result_one);
	let (people_and_me,my_relationships) = mi_mi_mi(people.clone(),entities);
	let arrangments = get_the_fucking_permutations(people_and_me.clone());
	let result_two= calculate_emotional_distress(arrangments.clone(),&my_relationships);
	println!("Solution part two = {:#?}",result_two);
}


fn mi_mi_mi(mut people:Vec<String>,mut aliens:Vec<HumanEntity>)->(Vec<String>,Vec<HumanEntity>){
	
	for p in &people {
		let me = HumanEntity {
				name:"me".to_string(),
				relationship_unit:0,
				target_name:p.clone(),
			};
		let not_me = HumanEntity{
		name:p.to_string(),
		relationship_unit:0,
		target_name:"me".to_string(),
		};
		aliens.push(me);
		aliens.push(not_me);
	}
	people.push("me".to_string());
	(people,aliens)
}

fn calculate_emotional_distress(communism:HashSet<Vec<String>>,aliens:&Vec<HumanEntity>)->i32{
	println!("please wait patiently, your answer is in progress ( i think)");
	let mut total_happyness:i32 = 0;

	for entry in communism{
		let mut happyness:i32 = 0;
		for name in entry.windows(2){
			for alien in aliens{
				if alien.name == name[0] && alien.target_name == name[1]{
					happyness += alien.relationship_unit;	
				}
				if alien.name == name[1] && alien.target_name == name[0]{
					happyness += alien.relationship_unit;	
				}
			}
		}	
		let len = entry.len()-1;
			for alien in aliens{
				if entry[0] == alien.name && entry[len] == alien.target_name || entry[len] == alien.name && entry[0] == alien.target_name{
					happyness += alien.relationship_unit;	
				}
			}	
		if happyness > total_happyness{
			total_happyness = happyness;
		}
	}
total_happyness
}

fn get_the_fucking_permutations(people:Vec<String>)->HashSet<Vec<String>>{

	let mut communism:HashSet<Vec<String>> = HashSet::new();
	communism.insert(people.clone());
    for i in 0..people.len() {
        let mut remaning = people.to_vec();
        remaning.remove(i);

        let sub_permutations = get_the_fucking_permutations(remaning);
        for mut sub_permutation in sub_permutations {
            sub_permutation.insert(0, people[i].clone());
            communism.insert(sub_permutation);
        }
    }
communism
}

fn nice_api(input:String)->(Vec<String>,Vec<HumanEntity>){
	let mut call_em_people:Vec<String> = Vec::new();
	let mut call_em_aliens:Vec<HumanEntity> = Vec::new();
	for entry in input.lines(){
		let mut words:Vec<_>= entry.split_whitespace().collect();
		words[10]= &words[10][0..words[10].len()-1];
		let identifier:String = words[0].to_string();
		let good_or_evil = words[2];
		let target = words[10];
		let mut happy_units :i32= words[3].parse().expect("everything is a number right?");
		if good_or_evil == "lose"{
			happy_units *= -1;
		}
		let person = HumanEntity{
			name: identifier.to_string(),
			relationship_unit:happy_units,
			target_name:target.to_string(),
		};
		if !call_em_people.contains(&identifier){call_em_people.push(identifier);}
		call_em_aliens.push(person);

	}	
(call_em_people,call_em_aliens)
}
