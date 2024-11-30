use std::convert::TryFrom;
use std::primitive::u8;

const ALPHA: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    let r = password_incrementer("hxbxwxba".to_string());
    println!("Solution to part one = {}", &r);
    let r_2 = password_incrementer(r);
    println!("Solution to part two = {}", &r_2);
}

fn password_incrementer(mut pw: String) -> String {
    let mut m = false;
    let mut bytes_pw: Vec<_> = pw.as_bytes().to_vec();
    let len = bytes_pw.len() - 1;
    while !m {
        'inner: loop {
            let mut count = 0;
            for c in bytes_pw.clone().iter().rev() {
                if (*c as u8) < 122 {
                    bytes_pw[len - count] += 1;
                    break 'inner;
                } else {
                    bytes_pw[len - count] -= 25;
                    count += 1;
                }
            }
        }
        let new_pw = std::str::from_utf8(&bytes_pw).expect("yes yes yes yes");
        m = validate(&new_pw);
        pw = new_pw.to_string();
    }
    pw
}

fn validate(pw: &str) -> bool {
    if pw.contains('i') || pw.contains('o') || pw.contains('l') {
        return false;
    }

    let fullfil_rec_two = validate_contains_doublicates(&pw);
    if fullfil_rec_two == false {
        return false;
    }

    let len = ALPHA.len() - 2;
    let mut n = 0;
    for _ in 0..len {
        let mut exu = String::new();
        let uwu = ALPHA.get(n..n + 3).expect("but why, BUT ????????");
        for c in uwu {
            exu.push(*c);
        }
        if pw.contains(&exu) {
            return true;
        }
        n += 1;
    }
    false
}

fn validate_contains_doublicates(pw: &str) -> bool {
    let mut result = false;
    let mut p = pw.chars();
    let p_len = pw.len() - 1;
    p.next();
    let mut double_counter: u8 = 0;
    let mut index_vecu: Vec<u8> = Vec::new();
    for (i, w) in pw.chars().take(p_len).enumerate() {
        let n: u8 = u8::try_from(i).ok().expect("the pw is only 8 chars lol");
        if w == p
            .next()
            .expect("what do I focus on next?, C?, Algorythms?, Rust?, Haskell?")
            && !index_vecu.contains(&n)
            && !index_vecu.contains(&(n + 1))
        {
            index_vecu.push(n);
            index_vecu.push(n + 1);
            double_counter += 1;
        }
    }
    if double_counter > 1 {
        result = true;
    }
    result
}
