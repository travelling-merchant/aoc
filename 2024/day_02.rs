fn main() {
    // witten dec 2014
    let file: String = String::from("day_02.txt");
    let input: String = std::fs::read_to_string(file).expect("snow snow snow");
    let r_one = x(&input);
    println!("Solution part one = {}", r_one);
}

fn x(input: &String) -> u16 {
	let mut grand_total:u16 = 0;
	for line in input.lines(){
	let mut v:Vec<u8> = Vec::new();
		let nums:Vec<_> = line.split_whitespace().collect();
		for entry in nums{
				let s:String = String::from(entry);
				let p:u8 = s.parse::<u8>().expect("just fuck you lol");
					v.push(p);
				 }
	
		let (is_valid,is_very_valid)= check_req(&v);
		if is_valid == true{
			grand_total +=1;
		}
	}
    grand_total
}


fn check_req(v:&Vec<u8>)->(bool,bool){
	let mut is_fine = false;
	let len = v.len();
	let mut is_counter_dec = 0;
	let mut is_counter_inc = 0;

	for (i,entry) in v[..len-1].iter().enumerate(){
		let n:u8 = v[i+1];
		let mut diff = 0;
		println!("n{}",n);	

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
		println!("counter {}",is_counter_dec);
		println!("counter {}",is_counter_inc);


is_fine
}

fn do_scary_math()->(u8,u8,){
	let mut c_o = 0;	
	let mut c_t = 0;	
}
