fn hamming(n: usize) -> u64 {
    if n == 1
	{
		return 1;
	}
	let mut min_value : u64 = 0;
	let mut vec = Vec::new();
	let mut count = 1;
	vec.push(1);
	while count <= n
	{
		println!("{:?} {}", vec, count);
		min_value = *vec.iter().min().unwrap() as u64;
		let max = u64::MAX;
		if (max / min_value) >= 2 && vec.contains(&(min_value * 2)) == false
		{
			vec.push(min_value * 2);
		}
		if (max / min_value) >= 3 && vec.contains(&(min_value * 3)) == false
		{
			vec.push(min_value * 3);
		}
		if (max / min_value) >= 5 && vec.contains(&(min_value * 5)) == false
		{
			vec.push(min_value * 5);
		}
		vec.remove(vec
			.iter()
			.position(|&x| x == min_value)
			.unwrap());
		count += 1;
		// if (conut + vec.len() > n)
		// 	min_value = hammer(vec)
	}
	min_value
}

fn main () {
	let ham = hamming(5000);
	// let mut i : u64= 2;
	// let mut count = 0;
	// loop
	// {
	// 	i = i * 2 as u64;
	// 	count += 1;
	// 	println!("{}", count);
	// }
	println!("{}", ham);
}