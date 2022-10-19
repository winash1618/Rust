fn partitions(n: u32) -> u32 {
	let n: i32 = n as i32;
	let mut vec = Vec::new();
	
	vec.push(1);
	let mut total = 0;
	let mut j = 1;
	while j <= n
	{
		let mut v = Vec::new();
		let mut ka = Vec::new();
		let mut i = 1;
		while i <= j
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
			if k * (3 * k - 1) / 2 > j
			{
				break;
			}
			v.push(j - k * (3 * k - 1) / 2);
			ka.push(r);
			i += 1;
		}
		total = 0;
		i = 0;
		while i < v.len() as i32
		{
			total += ka[i as usize] * vec[(v[i as usize])  as usize];
			i += 1;
		}
		j += 1;
		vec.push(total);
	}
	total as u32
}

fn main()
{
	// println!("1 = {}", partitions(1));
	// println!("2 = {}", partitions(2));
	// println!("3 = {}", partitions(3));
	// println!("4 = {}", partitions(4));
	// println!("5 = {}", partitions(5));
	// println!("6 = {}", partitions(6));
	// println!("7 = {}", partitions(7));
	// println!("8 = {}", partitions(8));
	// println!("9 = {}", partitions(9));
	// println!("10 = {}", partitions(10));
	// println!("11 = {}", partitions(11));
	// println!("12 = {}", partitions(12));
	// println!("13 = {}", partitions(13));
	// println!("14 = {}", partitions(14));
	// println!("15 = {}", partitions(15));
	// println!("16 = {}", partitions(16));
	// println!("17 = {}", partitions(17));
	// println!("18 = {}", partitions(18));
	// println!("19 = {}", partitions(19));
	// println!("20 = {}", partitions(20));
	// println!("21 = {}", partitions(21));
	// println!("22 = {}", partitions(22));
	// println!("23 = {}", partitions(23));
	// println!("24 = {}", partitions(24));
	// println!("25 = {}", partitions(25));
	// println!("26 = {}", partitions(26));
	// println!("27 = {}", partitions(27));
	// println!("28 = {}", partitions(28));
	// println!("29 = {}", partitions(29));
	// println!("30 = {}", partitions(30));
	// println!("31 = {}", partitions(31));
	// println!("32 = {}", partitions(32));
	// println!("33 = {}", partitions(33));
	// println!("34 = {}", partitions(34));
	// println!("35 = {}", partitions(35));
	// println!("36 = {}", partitions(36));
	// println!("37 = {}", partitions(37));
	// println!("38 = {}", partitions(38));
	// println!("39 = {}", partitions(39));
	// println!("40 = {}", partitions(40));
	// println!("41 = {}", partitions(41));
	// println!("42 = {}", partitions(42));
	// println!("43 = {}", partitions(43));
	// println!("44 = {}", partitions(44));
	// println!("45 = {}", partitions(45));
	// println!("46 = {}", partitions(46));
	// println!("47 = {}", partitions(47));
	// println!("48 = {}", partitions(48));
	// println!("49 = {}", partitions(49));
	// println!("50 = {}", partitions(50));
	// println!("51 = {}", partitions(51));
	// println!("52 = {}", partitions(52));
	// println!("53 = {}", partitions(53));
	// println!("54 = {}", partitions(54));
	// println!("55 = {}", partitions(55));
	// println!("56 = {}", partitions(56));
	// println!("57 = {}", partitions(57));
	// println!("58 = {}", partitions(58));
	// println!("59 = {}", partitions(59));
	// println!("60 = {}", partitions(60));
	// println!("61 = {}", partitions(61));
	// println!("62 = {}", partitions(62));
	// println!("63 = {}", partitions(63));
	// println!("64 = {}", partitions(64));
	// println!("65 = {}", partitions(65));
	// println!("66 = {}", partitions(66));
	// println!("67 = {}", partitions(67));
	// println!("68 = {}", partitions(68));
	// println!("69 = {}", partitions(69));
	// println!("70 = {}", partitions(70));
	// println!("71 = {}", partitions(71));
	// println!("72 = {}", partitions(72));
	// println!("73 = {}", partitions(73));
	// println!("74 = {}", partitions(74));
	// println!("75 = {}", partitions(75));
	// println!("76 = {}", partitions(76));
	// println!("77 = {}", partitions(77));
	// println!("78 = {}", partitions(78));
	// println!("79 = {}", partitions(79));
	// println!("80 = {}", partitions(80));
	// println!("81 = {}", partitions(81));
	// println!("82 = {}", partitions(82));
	// println!("83 = {}", partitions(83));
	// println!("84 = {}", partitions(84));
	// println!("85 = {}", partitions(85));
	// println!("86 = {}", partitions(86));
	// println!("87 = {}", partitions(87));
	// println!("88 = {}", partitions(88));
	// println!("89 = {}", partitions(89));
	// println!("90 = {}", partitions(90));
	// println!("91 = {}", partitions(91));
	// println!("92 = {}", partitions(92));
	// println!("93 = {}", partitions(93));
	// println!("94 = {}", partitions(94));
	// println!("95 = {}", partitions(95));
	// println!("96 = {}", partitions(96));
	// println!("97 = {}", partitions(97));
	// println!("98 = {}", partitions(98));
	// println!("99 = {}", partitions(99));
	println!("100 = {}", partitions(100));
}

// https://www.whitman.edu/mathematics/cgt_online/book/section03.03.html
// https://en.wikipedia.org/wiki/Pentagonal_number_theorem


/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

fn partitions(n: u32) -> u32 {
    fn p(k: u32, n: u32) -> u32 {
        if k > n { 0 }
        else if k == n { 1 }
        else { p(k + 1, n) + p(k, n - k) }
    }
    p(1, n)
}