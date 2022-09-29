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
    let sing = ["On the", "day of Christmas"];
    let days = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelfth",
    ];
    let love = "My true love gave to me";
    let mut i = 0;
    while i < 12 {
        let mut j : i32 = i;
		for (i, element) in song.iter().enumerate() {
			println!("INDEX = {}, element = {}", i, element);
			if ()
		}{
			let element = love[j];
            println!("{element}");
            j = j - 1;
        }
		println!("");
		i = i + 1;
    }
}
