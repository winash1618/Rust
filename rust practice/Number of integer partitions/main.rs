fn partitions(n: u32) -> u32 {
	let n: i32 = n as i32;
	if n == 0
	{
		return 1;
	}
	let mut sum = 0;
	if n > 0
	{
		let mut i = 1;
		while i <= n
		{
			let k;
			if i % 2 == 0
			{
				k = (i / 2) * -1;
			}
			else
			{
				k = (i + 1) / 2;
			}
			if k * (3 * k - 1) / 2 > n
			{
				break;
			}
			sum += partitions((n - k * (3 * k - 1) / 2) as u32);
			i += 1;
		}
	}
	println!("sum = {}", sum);
	sum
}

fn main()
{
	println!("n = {}", partitions(5));
}

// https://www.whitman.edu/mathematics/cgt_online/book/section03.03.html
// https://en.wikipedia.org/wiki/Pentagonal_number_theorem