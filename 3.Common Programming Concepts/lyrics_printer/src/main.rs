fn main() {
    let song = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
		"Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth", "tenth", "eleventh",
        "twelfth",
    ];
    for i in 0..12 {
		println!("On the {} day of Christmas", days[i]);
		println!("My true love gave to me:");
		for j in (0..i + 1).rev() {
			println!("{}", song[j]);
		}
		println!();
	}
}


