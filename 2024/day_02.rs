fn main() {
    // witten dec 2014
    let file: String = String::from("day_02.txt");
    let input: String = std::fs::read_to_string(file).expect("snow snow snow");
    let (r_one,v) = x(&input);
	let r_two = day_two_and_im_aleady_in_despair(v);
    println!("Solution part one = {}", r_one);
    println!("Solution part two = {}", r_two);
}

fn day_two_and_im_aleady_in_despair(v:Vec<Vec<u8>>)->u16{
let mut total_or_so = 0;

	for list in v{
		let mut bonus = true;
		let mut small_to_big:bool = false;
		let mut check_one = false;
		let mut count = 0;

		if list[0] < list[1]{
			small_to_big = true;	
		}
	
		let len = list.len()-1;	

		for (i,num) in list[0..len].into_iter().enumerate(){
			println!("entry is {}",num);
			let dif = get_dif(&num,&list[i+1]);
			if small_to_big && *num < list[i+1] && dif{
			println!("chesscake");
			count +=1;	
			}
			else if !small_to_big && *num > list[i+1] && dif{
			println!("strawberry cake");
			count +=1;	
			}
			else if bonus == true && i >0 {
				bonus = false;
				println!("num {}",num);
				let mut v:Vec<u8> = list.iter().map(|e | e.clone()).collect();
				v.remove(i);
				println!(" vectoru {:#?}",v);
				let is_valid = check_req(&v);
				if is_valid{
				count+=1;
				}else if !is_valid && i == len-1{
					print!("fuck you");
				let mut vv:Vec<u8> = list.iter().map(|e | e.clone()).collect();
				vv.remove(len);
				println!(" vectoru {:#?}",vv);
				let is_valid_too = check_req(&vv);
				println!("bgwiefbwiezfgo {}",is_valid_too);
				if is_valid_too{
				println!("bgwiefbwiezfgo {}",is_valid_too);
				count+=1;
				}
				}
			}
			else if i == 0 {
				let w = get_dif(&num,&list[i+2]);
				if  w{	
				count+=1;
				}
			}
		}
			println!("count is {}",count);
		if count == len{
			println!("count is {}",count);
			check_one = true;
		}
		
		if check_one == true{
		total_or_so +=1;
		}	
	}		
total_or_so
}

fn get_dif(n_1:&u8,n_2:&u8)->bool{
	let mut b = false;
	if n_1 > n_2{
		let r = n_1 - n_2;	
			if r < 4 && r >0{	b = true;}	
		}
	else {
		let r = n_2 - n_1 	;
		if r < 4 && r >0 {
			b = true;
		}
	}
b
}
fn x(input: &String) -> (u16,Vec<Vec<u8>>) {
	let mut grand_total:u16 = 0;
	let mut sad_meow:Vec<Vec<u8>> = Vec::new();
	for line in input.lines(){
	let mut v:Vec<u8> = Vec::new();
		let nums:Vec<_> = line.split_whitespace().collect();
		for entry in nums{
				let s:String = String::from(entry);
				let p:u8 = s.parse::<u8>().expect("just fuck you lol");
					v.push(p);
				 }
	
		let is_valid= check_req(&v);
		if is_valid == true{
			grand_total +=1;
		}
		sad_meow.push(v);
	}
    (grand_total,sad_meow)
}


fn check_req(v:&Vec<u8>)->bool{
	let mut is_fine = false;
	let len = v.len();
	let mut is_counter_dec = 0;
	let mut is_counter_inc = 0;

	for (i,entry) in v[..len-1].iter().enumerate(){
		let n:u8 = v[i+1];
		let mut diff = 0;

		if *entry > n{
			diff =	entry.checked_sub(n).expect("fml");
		}
		else{
			diff =	n.checked_sub(*entry).expect("aaaaaaahhhhh");	
		}
		

		if *entry >= n && diff < 4 && diff !=0{
			is_counter_dec +=1;	
		}	
		else if *entry <= n && diff < 4 && diff != 0 {
			is_counter_inc +=1;	
		}
	}
		if is_counter_dec == (len-1) || is_counter_inc == (len-1){ is_fine = true;}
is_fine
}
