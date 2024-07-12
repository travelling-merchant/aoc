// source for MD5 cookbook  https://www.ietf.org/rfc/rfc1321.txt

use std::fmt;
fn main(){
// test print
let x = denial("Coffee".to_string());
println!("test output step 1 {:#?}",x );
let (sol_one,sol_two) = do_big_math();
println!("Solution part one is {}",sol_one);
println!("Solution part two is {}",sol_two);
}
fn do_big_math()->(u32,u32){
let _puzzle_input = String::from("ckczppom");
let mut _count_1 = 0; 
let mut _count_2 = 0; 
// while hash(innput + count) not starts.with expected
// count_1 + 1
//return (count_1 ,count 2)
(0,0)
}


fn denial(input:String)->Vec<u8>{
// step 1 
// append bits 
// i don't know if this is nessecary but i only got it workign like this
// this adds one 1 bit wowies, btw did you knew 128 is 1000 0000 ?
let mut raw:Vec<u8> = input.into();
raw.push(0x80);

while (raw.len() * 8) % 512 != 448 {
// add + one 0 bit to the end
	raw.push(0x00);
}

raw
}
