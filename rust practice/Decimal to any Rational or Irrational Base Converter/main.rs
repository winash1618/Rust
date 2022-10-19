fn ft_float_convert(n: f64, decimals: u8, base: f64) -> String
{
	let num;
	if n = 0
	{
		num = (10).pow(decimals);
		mod = num % base;
		str = converter(num / base, decimals - 1, base)
	}
	let mod = 0;
	if num >= base
	{
		mod = num % base;
		str = converter(num / base, decimals, base)
	}
	str.push()
}

fn ft_convert(n: f64, decimals: u8, base: f64) -> String
{
	let vec = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'] ;
	let num = n * (10).pow(decimals);
	let mod = 0;
	if num >= base
	{
		mod = num % base;
		str = converter(num / base)
	}
	str.push(vec[mod])
}
fn converter(n: f64, decimals: u8, base: f64) -> String {
	let mut str = String::new();
	if base - base as i64 > 0
	{
		str = ft_float_convert(n, decimals, base);
	}
	else
	{
		str = ft_convert(n, decimals, base);
	}
	return (str);
}

fn main()
{

}