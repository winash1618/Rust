// fn ft_float_convert(n: f64, decimals: u8, base: f64) -> String
// {

// 	let num;
// 	if n = 0
// 	{
// 		num = (10).pow(decimals);
// 		mod = num % base;
// 		str = converter(num / base, decimals - 1, base)
// 	}
// 	let mod = 0;
// 	if num >= base
// 	{
// 		mod = num % base;
// 		str = converter(num / base, decimals, base)
// 	}
// 	str.push()
// }

fn ft_convert(n: f64, decimals: u8, base: f64) -> String
{
	let vec = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'] ;
	let num = n * (10_i32).pow(decimals as u32) as f64;
	let mut mods : f64 = 0.0;
	let mut s = String::new();
	if num >= base
	{
		mods = num % base;
		println!("{} {} {} {}",mods, n, decimals, base);
		s = ft_convert((num - mods) / base, decimals, base);
	}
	s.push(vec[mods as usize]);
	
	s
}
fn converter(n: f64, decimals: u8, base: f64) -> String {
	
	
	let s;
	// if base - (base as i64) as f64 > 0.0
	// {
	// 	str = ft_float_convert(n, decimals, base);
	// }
	// else
	// {
		s = ft_convert(n, decimals, base);
		println!("string = {}", s);
	// }
	return s;
}

fn main()
{
	// converter(13.0, 0, std::f64::consts::PI);
	converter(13.0, 0, 2.0);
}