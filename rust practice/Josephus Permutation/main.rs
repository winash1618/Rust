fn josephus_recursion<T:Clone+Copy + std::fmt::Debug>(xs:Vec<T>, k:usize, loc:usize) -> Vec<T> {
    let mut vec = Vec::new();
	let mut rem = k;
    let mut loc = loc;
    let mut xs = xs;
    
    if xs.len() > 0
	{
		if xs.len() < k
		{
			println!("1loc {} rem {} len {}", loc, rem, xs.len());
			rem = k % xs.len();
		}
		if loc == 0 && rem == 0
		{
			println!("2loc {} rem {} len {}", loc, rem, xs.len());
			loc = xs.len() - 1;
		}
		else if (loc + rem) > xs.len()
		{
			println!("3loc {} rem {} len {}", loc, rem, xs.len());
			loc = (loc + rem) - xs.len() - 1;
		}
		else
		{
			println!("4loc {} rem {} len {}", loc, rem, xs.len());
			loc = loc + rem;
			loc = loc - 1;
		}
		vec.push(xs[loc]);
		xs.remove(loc);
		if xs.len() > 0
		{
			if xs.len() == loc
			{
				println!("5loc {} rem {} len {}", loc, rem, xs.len());
				loc = 0;
			}
			vec.extend(josephus_recursion(xs, k, loc));
		}
	}
	// else if xs.len() == 1 
	// {
	// 	vec.push(xs[0]);
	// 	xs.remove(loc);
	// }
	vec
}

fn josephus<T:Clone+Copy + std::fmt::Debug>(xs:Vec<T>,k:usize)-> Vec<T> {
    
    println!("{:?}", xs);
	let v = josephus_recursion(xs, k, 0);
	println!("{:?}", v);
	v
}

fn main()
{
	josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1);
	josephus("CodeWars".chars().collect::<Vec<char>>(), 4);
}