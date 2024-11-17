const ALPHA: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    let r = idk();
    println!("{:#?}", r);
}

fn idk() -> bool {
    //let pw: &str = "hxbxwxba";
    let pw: &str = "ghjaabcc";
    let m = validate(pw);
    m
}
fn validate(pw: &str) -> bool {
	println!("validating password");
    if pw.contains('i') || pw.contains('o') || pw.contains('l') {
        return false;
    } else {
		println!("password check 1 / 3 success");
    }
    let mut p = pw.chars();
	let p_len = pw.len()-2;
    p.next();
    if !pw
        .chars().take(p_len)
        .any(|c| c == p.next().expect("tired?"))
    {
        return false;
    } else {
		println!("password check 2 / 3 success");
    }
let len = ALPHA.len()-2;
let mut n = 0;
    for _ in 0..len{
		let mut exu = String::new();
		let uwu = ALPHA.get(n..n+3).expect("but why, BUT ????????");
		for c in uwu{
		exu.push(*c).to_owned();
		}
        if pw.contains(&exu) {
		println!("password check 3 / 3 success");
            return true;
        }
		n +=1;
	}
false
}
