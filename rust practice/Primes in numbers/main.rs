fn prime_factors(n: i64) -> String {
	let mut str = String::new();
	let mut count;
	let mut i = 2;
	let mut n = n;
	while n > 1
	{
		count = 0;
		while n % i == 0
		{
			count += 1;
			n /= i;
		}
		// println!("{}", count);
		if count > 1
		{
			str.push_str("(");
			str.push_str(&(i.to_string()));
			str.push_str("**");
			str.push_str(&(count.to_string()));
			str.push_str(")");
		}
		else if count == 1
		{
			str.push_str("(");
			str.push_str(&(i.to_string()));
			str.push_str(")");
		}
		i += 1;
	}
	str
}

fn main()
{
	let str = prime_factors(4665464);
	// println!("{}", str);
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn prime_factors(n: i64) -> String {
//     let mut number = n; 
//     let mut prime_numbers = vec![];
//     let mut factor = 2;
//     let mut cnt;
//     while number > 1 {
//         cnt = 0;
//         while number % factor == 0 {
//             number /= factor;
//             cnt += 1;
//         }
//         if cnt > 0 {
//             if cnt > 1
//                 {prime_numbers.push(format!("({}**{})", factor, cnt));}
//             else
//                 {prime_numbers.push(format!("({})", factor));}
//         }
//         factor += 1;
//     }
//     prime_numbers.join("")
// }

// fn prime_factors(n: i64) -> String {
// 	let mut n = n as u64;
// 	let mut d = 2;
// 	let mut mem = std::collections::BTreeMap::new();
// 	while d <= n {
// 	  if n % d == 0 {
// 		n /= d;
// 		let old = mem.entry(d).or_insert(0);
// 		*old += 1;
// 	  } else {
// 		d += 1;
// 	  }
// 	}
// 	mem.iter().map(|(key, val)| match *val {
// 	  1 => format!("({})", key),
// 	  _ => format!("({}**{})", key, val),
// 	}).collect::<String>()
//   }