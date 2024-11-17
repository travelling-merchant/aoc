const ALPHA: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    let r = password_incrementer();
    println!("{:#?}", r);
}

fn password_incrementer() -> &'static str{
	// old password
    let pw: &str = "hxbxwxba";
	//valid pw for test
    //let pw: &str = "ghjaabcc";
	let mut m = false;
	let mut bytes_pw:Vec<_> = pw.as_bytes().to_vec();
	while !m{
		'inner: loop{
		let mut count =0;
		for  c in bytes_pw.clone().iter().rev(){
			if *c as u8 != 122{ bytes_pw[count] += 1 ;break 'inner;}
			
			else { bytes_pw[count] = 97;count +=1;}
			}
			}
		let new_pw = std::str::from_utf8(&bytes_pw).expect("yes yes yes yes");
		println!("potental new pw => {}",&new_pw);
   		m = validate(&new_pw);
	}
    pw
}

fn validate(pw: &str) -> bool {
    if pw.contains('i') || pw.contains('o') || pw.contains('l') {
        return false;
    }
    let mut p = pw.chars();
	let p_len = pw.len()-2;
    p.next();
    if !pw
        .chars().take(p_len)
        .any(|c| c == p.next().expect("tired?"))
    {
        return false;
    }
let len = ALPHA.len()-2;
let mut n = 0;
    for _ in 0..len{
		let mut exu = String::new();
		let uwu = ALPHA.get(n..n+3).expect("but why, BUT ????????");
		for c in uwu{
		exu.push(*c);
		}
        if pw.contains(&exu) {
		println!("password check 3 / 3 success");
            return true;
        }
		n +=1;
	}
false
}
