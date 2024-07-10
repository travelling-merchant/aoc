use std::fs;
#[derive(Debug,Clone,PartialEq)]
struct Grid{
	x:i32,
	y:i32,
}

fn main(){
let (s,_x,_v) = p_one();
print!("solution to part one is {:#?}",s.len());
}


fn p_one()->(Vec<Grid>,String,u32){
let s = 0;
let mut visited:Vec<Grid> = Vec::new();
let default = Grid{x:0,y:0};
let mut x = 0;
let mut y = 0;
visited.push(default);
let instructions = get_file_cont("day_3.txt".to_string());
for symbol in instructions.chars(){
	let mut town = Grid{x:0,y:0};
	if symbol == '<'{
	x-=1; 
	town.x =x;	
	town.y =y;	
	visited.push(town);
	}
	else if symbol == '>'{
	x+=1; 
	town.x =x;	
	town.y =y;	
	visited.push(town);
	}
	else if symbol == '^'{
	y+=1; 
	town.x =x;	
	town.y =y;	
	visited.push(town);
	}
	else if symbol == 'v'{
	y-=1; 
	town.x =x;	
	town.y =y;	
	visited.push(town);
	}
}

let mut stalker:Vec<Grid> = Vec::new();
for v in visited{
	if !stalker.contains(&v){
		stalker.push(v);
		}
	}
(stalker,instructions, s)
}

fn get_file_cont(i:String)->String{
let s = fs::read_to_string(i).expect("you forgot the file btw...");
s
}
