#[derive(PartialEq,Debug)]
struct HumanEntity{
	name:String,
	relationship_unit:i32,
	target_name:String,
}
fn main(){
	let input = std::fs::read_to_string("day_13.txt".to_string()).expect("still pretty dark outisde");
	let result = nice_api(input);
	println!("{:#?}",result);
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
