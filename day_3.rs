use std::fs;
#[derive(Debug,Clone,PartialEq)]
struct Grid{
	x:i32,
	y:i32,
}

fn main(){
let s = p_one();
let two = p_two();
println!("solution to part one is {:#?}",s.len());
println!("solution to part two is {:#?}",two);
}
fn p_two()->usize{
let instructions = get_file_cont("day_3.txt".to_string());
let (v_1,v_2) = loop_ins(instructions);
let mut sol_two:Vec<Grid> = Vec::new();
for meow in v_1{
if !sol_two.contains(&meow){
	sol_two.push(meow);
	}
}
for meow in v_2{
if !sol_two.contains(&meow){
	sol_two.push(meow);
	}
}
sol_two.len()
}

fn loop_ins(ins:String)->(Vec<Grid>,Vec<Grid>){
	let mut c = 0;
	let mut x = 0;
	let mut y = 0;
	let mut x_2 = 0;
	let mut y_2 = 0;

	let mut v_1:Vec<Grid> = Vec::new();
	let mut v_2:Vec<Grid> = Vec::new();
	let g = Grid{x:0,y:0};
	v_2.push(g.clone());
	v_1.push(g);
	for e in ins.chars(){
		let mut town = Grid{x:0,y:0};
		if c % 2 == 0{
			if e == '<'{
			x-=1; 
			town.y =y;
			town.x =x;
			v_2.push(town);
			}
			else if e == '>'{
			x+=1;
			town.y =y;
			town.x =x;
			v_2.push(town);
			}
			else if e == '^'{
			y+=1;
			town.y =y;
			town.x =x;
			v_2.push(town);
			}
			else if e == 'v'{
			y-=1;
			town.y =y;
			town.x =x;
			v_2.push(town);
			}	
		}
		else {
			if e == '<'{
			x_2 -= 1; 
			town.y =y_2;
			town.x =x_2;
			v_2.push(town);
			}
			else if e == '>'{
			x_2 +=1;
			town.y =y_2;
			town.x =x_2;
			v_2.push(town);
			}
			else if e == '^'{
			y_2 +=1;
			town.y =y_2;
			town.x =x_2;
			v_2.push(town);
			}
			else if e == 'v'{
			y_2 -=1;
			town.y =y_2;
			town.x =x_2;
			v_2.push(town);
			}
		}
		c +=1 ;
	}
(v_2,v_1)
}
fn p_one()->Vec<Grid>{
let mut x = 0;
let mut y = 0;
let default = Grid{x:0,y:0};
let mut  visited:Vec<Grid> = Vec::new();
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

stalker
}

fn get_file_cont(i:String)->String{
let s = fs::read_to_string(i).expect("you forgot the file btw...");
s
}
