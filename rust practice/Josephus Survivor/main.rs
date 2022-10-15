fn josephus_survivor(n: i32, k: i32) -> i32 {
	if n > 1
	{
		return ((josephus_survivor(n - 1, k) + k - 1) % n) + 1;
	}
	return 1;
}

fn main()
{
	let n = josephus_survivor(7, 3);
	println!("{}", n);
}