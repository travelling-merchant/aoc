// source for MD5 cookbook  https://www.ietf.org/rfc/rfc1321.txt

fn main(){
// test print
let x = circle_of_life("Big Coffee lalalalalaalalalalalalallaalalalalal".to_string());
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
 // lol there was no bargaining imagine having a saying, are you kidding
 let _stage_three = bargaining ();
 let d = depression(stage_two);
 //stage_four
let m:Vec<u8> = Vec::new();
m
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


#[derive(Debug)]
struct MD5Buffer{
a:u32,
b:u32,
c:u32,
d:u32
}
// I SWEAR AT THE END EVERYTHING IS A FUNCTION
// IS IMPL REALLY THAT MUCH BETTER
// FUCK EVERYONE ,REALLY
// IMPL IS JUST A UNJUSTLY GLORRIFIED FUNCTION
fn bargaining()->MD5Buffer{
	let r = MD5Buffer{
	a:0x67452301,
	b:0xefcdab89,
	c:0x98badcfe,
	d:0x10325476,
	};
	r
}
impl MD5Buffer{
	fn new() ->Self{
		MD5Buffer{
		a:0x67452301,
		b:0xefcdab89,
		c:0x98badcfe,
		d:0x10325476,
		}
	}
}


fn depression(process_msg:Vec<u8>)->Vec<u32>{
	let mut buffer = MD5Buffer::new();
	// just wtf am I reading in the MD 5 Manual??
	// whatever happens in step 4 in the guide should be like 4 different steps!
	// first we create some fun litle bit operator things, easy
	fn capital_f(x:u32,y:u32,z:u32)->u32{ 
	(x & y) | (!x & z)
	}

	fn capital_g(x:u32,y:u32,z:u32)->u32{ 
	(x & z) | (y & !z)
	}

	fn capital_h(x:u32,y:u32,z:u32)->u32{ 
	x ^ y ^ z
	}

	fn capital_i(x:u32,y:u32,z:u32)->u32{ 
	y ^ (x | !z)
	}
	
	// turn vec u8 into vec u32 for next steps owo
	let mut words: Vec<u32> = Vec::new();
	let mut buffer:u32 = 0;
	let mut byte_index = 0;
	
	for byte in process_msg.iter(){
	buffer = buffer << 8;
	buffer |= *byte as u32;
	byte_index +=1;
		if byte_index % 4 == 0{
		words.push(buffer);
		buffer = 0;	
		}
	}

	println!("words{:#?}",words);
	// get da complicated math
	let t = acceptance();
	println!("math{:?}",t);

	for block_index in 0..words.len()/16{ // diveded by 16 only sets the amount of iterations right
		let block_words = &words[block_index * 16..block_index * 16 + 16]; //  this actually creates the 16 word / bit block, I hope

	aa = buffer.a;
	bb = buffer.b;
	cc = buffer.c;
	dd = buffer.d;
	}
	

words
}

// I was sleeping during math back in school
// so no idea what im doing
// why did no one told me math can be beautiful
fn acceptance()->[u32;64]{
// docs say this is called t even thos its not a table anymore
// because we strong
let mut t = [0u32;64];
	for i in 0..64{
	// this number is in docs, don't ask me, me = noob
	t[i] = (4294967296.0 * ( i as f64 + 1.0).sin().abs()).floor() as u32;
	}
	t

}
