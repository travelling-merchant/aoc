fn main() {
    let secret = String::from("ckczppom");
    let mut counter_one = 0;
    let mut counter_two = 0;

    loop {
        let secret_with_num = format!("{}{}", secret, counter_one);
        let hash_result = fuck_md5(secret_with_num);
        if hash_result.starts_with("00000") {
            break;
        } else {
            counter_one += 1;
        }
    }
    loop {
        let secret_with_num = format!("{}{}", secret, counter_two);
        let hash_result = fuck_md5(secret_with_num);
        if hash_result.starts_with("000000") {
            break;
        } else {
            counter_two += 1;
        }
    }
    println!("Solution to part one is {}", counter_one);
    println!("Solution to part two is {}", counter_two);
}

fn fuck_md5(input: String) -> String {
    let (result_one, result1) = step_one(input);
    let result_two = step_two(result_one, result1);
    let result_four = step_four(result_two);

    let mut result = String::from("");
    for &entry in result_four.iter() {
        result.push_str(&format!("{:02x}", entry));
    }
    result
}
fn step_one(input: String) -> (Vec<u8>, usize) {
    let l = input.len();
    let mut r: Vec<u8> = input.into();
    r.push(0x80);
    while r.len() * 8 % 512 != 448 {
        r.push(0x00);
    }
    (r, l)
}
fn step_two(mut input: Vec<u8>, msg_len: usize) -> Vec<u8> {
    let l = msg_len as u64 * 8;
    for b in l.to_le_bytes().iter() {
        input.push(*b);
    }
    input
}
const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

fn step_four(input: Vec<u8>) -> [u8; 16] {
    let mut state = [A, B, C, D];
    let t = t();
    for i in 0..input.len() / 64 {
        let mut x = [0u32; 16];
        for j in 0..16 {
            let location = i * 64 + j * 4;
            x[j] = u32::from_le_bytes([
                input[location],
                input[location + 1],
                input[location + 2],
                input[location + 3],
            ]);
        }
        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];

        let s: [u32; 16] = [7, 12, 17, 22, 5, 9, 14, 20, 4, 11, 16, 23, 6, 10, 15, 21];
        // round one (I have the desire to this with a for loop, just to make it beautiful but i wont
        // if this doesent work I will change it to a for loop tho
        magic_one(f, &mut a, b, c, d, x[0], s[0], t[0]);
        magic_one(f, &mut d, a, b, c, x[1], s[1], t[1]);
        magic_one(f, &mut c, d, a, b, x[2], s[2], t[2]);
        magic_one(f, &mut b, c, d, a, x[3], s[3], t[3]);

        magic_one(f, &mut a, b, c, d, x[4], s[0], t[4]);
        magic_one(f, &mut d, a, b, c, x[5], s[1], t[5]);
        magic_one(f, &mut c, d, a, b, x[6], s[2], t[6]);
        magic_one(f, &mut b, c, d, a, x[7], s[3], t[7]);

        magic_one(f, &mut a, b, c, d, x[8], s[0], t[8]);
        magic_one(f, &mut d, a, b, c, x[9], s[1], t[9]);
        magic_one(f, &mut c, d, a, b, x[10], s[2], t[10]);
        magic_one(f, &mut b, c, d, a, x[11], s[3], t[11]);

        magic_one(f, &mut a, b, c, d, x[12], s[0], t[12]);
        magic_one(f, &mut d, a, b, c, x[13], s[1], t[13]);
        magic_one(f, &mut c, d, a, b, x[14], s[2], t[14]);
        magic_one(f, &mut b, c, d, a, x[15], s[3], t[15]);

        // round 2
        magic_one(g, &mut a, b, c, d, x[1], s[4], t[16]);
        magic_one(g, &mut d, a, b, c, x[6], s[5], t[17]);
        magic_one(g, &mut c, d, a, b, x[11], s[6], t[18]);
        magic_one(g, &mut b, c, d, a, x[0], s[7], t[19]);

        magic_one(g, &mut a, b, c, d, x[5], s[4], t[20]);
        magic_one(g, &mut d, a, b, c, x[10], s[5], t[21]);
        magic_one(g, &mut c, d, a, b, x[15], s[6], t[22]);
        magic_one(g, &mut b, c, d, a, x[4], s[7], t[23]);

        magic_one(g, &mut a, b, c, d, x[9], s[4], t[24]);
        magic_one(g, &mut d, a, b, c, x[14], s[5], t[25]);
        magic_one(g, &mut c, d, a, b, x[3], s[6], t[26]);
        magic_one(g, &mut b, c, d, a, x[8], s[7], t[27]);

        magic_one(g, &mut a, b, c, d, x[13], s[4], t[28]);
        magic_one(g, &mut d, a, b, c, x[2], s[5], t[29]);
        magic_one(g, &mut c, d, a, b, x[7], s[6], t[30]);
        magic_one(g, &mut b, c, d, a, x[12], s[7], t[31]);

        // round 3
        magic_one(h, &mut a, b, c, d, x[5], s[8], t[32]);
        magic_one(h, &mut d, a, b, c, x[8], s[9], t[33]);
        magic_one(h, &mut c, d, a, b, x[11], s[10], t[34]);
        magic_one(h, &mut b, c, d, a, x[14], s[11], t[35]);

        magic_one(h, &mut a, b, c, d, x[1], s[8], t[36]);
        magic_one(h, &mut d, a, b, c, x[4], s[9], t[37]);
        magic_one(h, &mut c, d, a, b, x[7], s[10], t[38]);
        magic_one(h, &mut b, c, d, a, x[10], s[11], t[39]);

        magic_one(h, &mut a, b, c, d, x[13], s[8], t[40]);
        magic_one(h, &mut d, a, b, c, x[0], s[9], t[41]);
        magic_one(h, &mut c, d, a, b, x[3], s[10], t[42]);
        magic_one(h, &mut b, c, d, a, x[6], s[11], t[43]);

        magic_one(h, &mut a, b, c, d, x[9], s[8], t[44]);
        magic_one(h, &mut d, a, b, c, x[12], s[9], t[45]);
        magic_one(h, &mut c, d, a, b, x[15], s[10], t[46]);
        magic_one(h, &mut b, c, d, a, x[2], s[11], t[47]);

        // round 4
        magic_one(fu_i, &mut a, b, c, d, x[0], s[12], t[48]);
        magic_one(fu_i, &mut d, a, b, c, x[7], s[13], t[49]);
        magic_one(fu_i, &mut c, d, a, b, x[14], s[14], t[50]);
        magic_one(fu_i, &mut b, c, d, a, x[5], s[15], t[51]);

        magic_one(fu_i, &mut a, b, c, d, x[12], s[12], t[52]);
        magic_one(fu_i, &mut d, a, b, c, x[3], s[13], t[53]);
        magic_one(fu_i, &mut c, d, a, b, x[10], s[14], t[54]);
        magic_one(fu_i, &mut b, c, d, a, x[1], s[15], t[55]);

        magic_one(fu_i, &mut a, b, c, d, x[8], s[12], t[56]);
        magic_one(fu_i, &mut d, a, b, c, x[15], s[13], t[57]);
        magic_one(fu_i, &mut c, d, a, b, x[6], s[14], t[58]);
        magic_one(fu_i, &mut b, c, d, a, x[13], s[15], t[59]);

        magic_one(fu_i, &mut a, b, c, d, x[4], s[12], t[60]);
        magic_one(fu_i, &mut d, a, b, c, x[11], s[13], t[61]);
        magic_one(fu_i, &mut c, d, a, b, x[2], s[14], t[62]);
        magic_one(fu_i, &mut b, c, d, a, x[9], s[15], t[63]);

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
    }
    let mut result = [0u8; 16];
    for (i, &val) in state.iter().enumerate() {
        result[i * 4..(i + 1) * 4].copy_from_slice(&val.to_le_bytes());
    }
    result
}

fn magic_one(
    func: fn(u32, u32, u32) -> u32,
    a: &mut u32,
    b: u32,
    c: u32,
    d: u32,
    x: u32,
    s: u32,
    t: u32,
) {
    let temp_v = a
        .wrapping_add(func(b, c, d))
        .wrapping_add(x)
        .wrapping_add(t);
    *a = b.wrapping_add(temp_v.rotate_left(s));
}
fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn fu_i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}
//fn t()->[u32;64]{
//let mut t = [0u32;64];
//for i in 0..64{
//t[i] = (4294967296.0 * ( i as f64 + 1.0).sin().abs()).floor() as u32;
//}
//t
//}

fn t() -> [u32; 64] {
    [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ]
}
