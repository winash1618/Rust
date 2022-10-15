

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
		// println!("{:?}", vec);
		min_value = *vec.iter().min().unwrap() as u64;
		if vec.contains(&(min_value * 2)) == false
		{
			vec.push(min_value * 2);
		}
		if vec.contains(&(min_value * 3)) == false
		{
			vec.push(min_value * 3);
		}
		if vec.contains(&(min_value * 5)) == false
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
	println!("{}", ham);
}