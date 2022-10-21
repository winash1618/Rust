fn converter(n: f64, decimals: u8, base: f64) -> String {
    let vec = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'] ;
    let mut num = n;
    let mut i = 0;
    let mut b = base;
    let mut s = String::new();
    let mut decimals = decimals;
    if num < 0.0
    {
        s.push('-');
        num = -1.0 * num;
    }
    while num >= b
    {
        b = b * base;
        i += 1;
    }
    b = b / base;
    while i >= 0
    {
        s.push(vec[(num / b)as usize]);
        num = num - b * ((num / b)as i64)as f64;
        b = b / base;
        i -= 1;
    }
    if decimals > 0
    {
        s.push('.');
    }
    while decimals > 0 
    {
        let mods : f64;
        mods = num * base;
        s.push(vec[mods as usize]);
        decimals -= 1;
        num = mods - ((mods as usize)as f64);
    }
    return s;
}


fn main()
{
	// converter(13.0, 0, std::f64::consts::PI);
	converter(13.0, 3, std::f64::consts::PI);
	// converter(13.0, 4, 2.0);
	// converter(-15.5, 2, 23.0);
	// converter(13.0, 0, 10.0);
}