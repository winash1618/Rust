fn second_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	let mut k = 0;
	let mut j = 0;
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' && k == 0{
			k = 1;
			j = i;
		}
		else if item == b' ' && k == 1 {
			return &s[j + 1..i];
		}
	}
	if k == 1{
		return &s[j + 1..];}
	&s[..]
}

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}
fn main() {
    let s = String::from("hello world");
	let first = first_word(&s);
    let word = second_word(&s); // word will get the value 5
	println!("{first}");
	println!("{word}");
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}