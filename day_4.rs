// source for MD5 cookbook  https://www.ietf.org/rfc/rfc1321.txt

fn main(){
// test print
let x = circle_of_life("Coffee".to_string());
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



fn circle_of_life(input:String)->Vec<u8>{
 let you_mad = input.len();
 let stage_one = denial(input);
 let stage_two = anger(stage_one,you_mad);
stage_two
}


fn denial(input:String)->Vec<u8>{
// step 1 
// append bits 
// i don't know if this is nessecary but i only got it workign like this
// this adds one 1 bit wowies, btw did you knew 128 is 1000 0000 ?
let mut raw:Vec<u8> = input.into();
raw.push(0x80);

// len of bits so 8 times for big BYTE and md5 says needs to be 448 if modulo 512, I think
// you can also try this future me,
// while raw.len() & 0x7 != 1{ // bitwise shenanigans, maybe faster
while (raw.len() * 8) % 512 != 448 {
// add + one 0 bit to the end
	raw.push(0x00);
}

raw
}

fn anger(mut input_with_append:Vec<u8>,o_msg:usize)->Vec<u8>{

// docs speci we append the lengh od the msg to step 1
// so len of msg * 8 because else we only have the amount of bytes / chars 
let len_i_b = (o_msg as u64) * 8;

// to the fancy thing turn in bites with confusing terms (little-endian) why that name?
let len_b = len_i_b.to_le_bytes();

// you knwo loops, good, I dont
for byte in len_b.iter(){
input_with_append.push(*byte);
}
input_with_append
}
