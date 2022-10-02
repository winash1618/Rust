fn main() {
    let config_max = Some(3u8);// Here 3 is a number and u8 is type
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
