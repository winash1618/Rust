use std::io;
fn main() {
    println!("Farenheit to Celsius Converter");
    loop {
        println!("Enter temperature in Farenheit : ");
        let mut farenheit = String::new();
        io::stdin()
            .read_line(&mut farenheit)
            .expect("Failed to read line");
        let farenheit: f32 = match farenheit.trim().parse() {
            Ok(num) => num, // Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value.
            Err(_) => continue, // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values.
                                // continue, which tells the program to go to the next iteration of the loop and ask for another guess.
        };
        let celsius: f32 = (farenheit - 32.0) * (5.0 / 9.0);
        println!("Temperature in celsius: {celsius}");
    }
}
