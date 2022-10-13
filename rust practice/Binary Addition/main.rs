fn add_binary(a: u64, b: u64) -> String {
    let mut str_bits = String::new();
    let mut i : u64 = a + b;
    while i > 0
    {
        if i % 2 == 0
        {
            str_bits.push_str("0");
        }
        else
        {
            str_bits.push_str("1");
        }
        i /= 2;
    }
    str_bits.chars().rev().collect::<String>()
}