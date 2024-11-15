const ALPHA: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    let r = idk();
    println!("{:#?}", r);
}

fn idk() -> bool {
    let pw: &str = "hxbxwxba";
    let m = validate(pw);
    m
}
fn validate(pw: &str) -> bool {
    let mut truth_counter = 0;
    if pw.contains('i') || pw.contains('j') || pw.contains('l') {
        return false;
    } else {
        truth_counter += 1;
    }
    let mut p = pw.chars();
	let p_len = pw.len()-2;
    p.next();
// overflow
    if !pw
        .chars().take(p_len)
        .any(|c| c == p.next().expect("tired?"))
    {
        return false;
    } else {
        truth_counter += 1;
    }
let len = ALPHA.len()-2;
let mut n = 0;
    for _ in 0..len{
		let three_letter_sequence:String = format!("{:?}",ALPHA.get(n..n+2).expect("but why, BUT ????????"));
		println!("{}",&three_letter_sequence);
        if pw.contains(&three_letter_sequence) {
            return true;
        }
		else {return false}
		n +=1;
	}
false
}
