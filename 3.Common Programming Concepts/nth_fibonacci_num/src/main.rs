use std::io;

fn main() {
    println!("Nth Fibonacci Number");
    let num = loop {
        println!("Please enter N = ");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        let num: u32 = match num.trim().parse() {
            Ok(n) => n, // Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value.
            Err(_) => continue, // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values.
                                // continue, which tells the program to go to the next iteration of the loop and ask for another guess.
        };
        break num;
    };
	println!("who are you {num}");
    let fib_n = {
        let mut first = 0;
        let mut second = 0;
        let mut total = 0;
        let mut i = 1;
        while i < num {
            if i == 1 {
                first = 1;
                second = 0;
                total = 1;
            } else if i == 2 {
                first = 1;
                second = 1;
                total = 1;
            } else {
                total = first + second;
                second = first;
                first = total;
            }
            i += 1;
        }
        total
    };
    println!("fibonacci number {fib_n}");
}
