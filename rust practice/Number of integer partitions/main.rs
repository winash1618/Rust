fn partitions(n: u32) -> u32 {
	let n: i32 = n as i32;
	if n == 1 || n == 0
	{
		return 1;
	}
	if n == 2
	{
		return 2;
	}
	if n == 3
	{
		return 3;
	}
	if n == 4
	{
		return 5;
	}
	if n == 5
	{
		return 7;
	}
	let mut sum : i32 = 0;
	if n > 5
	{
		let mut i = 1;
		while i <= n
		{
			let k;
			let r;
			if i % 2 == 0
			{
				k = (i / 2) * -1;
				if (k - 1) % 2 == 0
				{
					r = 1;
				}
				else
				{
					r = -1;
				}
			}
			else
			{
				k = (i + 1) / 2;
				if (-1 * (k - 1)) % 2 == 0
				{
					r = 1;
				}
				else
				{
					r = -1;
				}
			}
			if k * (3 * k - 1) / 2 > n
			{
				break;
			}
			sum += r * partitions((n - k * (3 * k - 1) / 2) as u32) as i32;
			// println!("n = {} sum = {}", n, sum);
			i += 1;
		}
	}
    
	sum as u32
}

fn main()
{
	println!("1 = {}", partitions(1));
	println!("2 = {}", partitions(2));
	println!("3 = {}", partitions(3));
	println!("4 = {}", partitions(4));
	println!("5 = {}", partitions(5));
	println!("6 = {}", partitions(6));
	println!("7 = {}", partitions(7));
	println!("8 = {}", partitions(8));
	println!("9 = {}", partitions(9));
	println!("10 = {}", partitions(10));
	println!("11 = {}", partitions(11));
	println!("12 = {}", partitions(12));
	println!("13 = {}", partitions(13));
	println!("14 = {}", partitions(14));
	println!("15 = {}", partitions(15));
	println!("16 = {}", partitions(16));
	println!("17 = {}", partitions(17));
	println!("18 = {}", partitions(18));
	println!("19 = {}", partitions(19));
	println!("20 = {}", partitions(20));
	println!("21 = {}", partitions(21));
	println!("22 = {}", partitions(22));
	println!("23 = {}", partitions(23));
	println!("24 = {}", partitions(24));
	println!("25 = {}", partitions(25));
	println!("26 = {}", partitions(26));
	println!("27 = {}", partitions(27));
	println!("28 = {}", partitions(28));
	println!("29 = {}", partitions(29));
	println!("40 = {}", partitions(40));
}

// https://www.whitman.edu/mathematics/cgt_online/book/section03.03.html
// https://en.wikipedia.org/wiki/Pentagonal_number_theorem