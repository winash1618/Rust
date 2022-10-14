fn josephus_recursion<T:Clone+Copy>(xs:Vec<T>, k:usize, l:usize) -> Vec<T> {
    let mut vec = Vec::new();
	let mut rem = k;
    if xs.len() > 0
	{
		if (xs.len() < k)
		{
			rem = k % xs.len();
		}
		if loc + rem >= xs.len()
		{
			loc += rem - xs.len();
		}
		else
		{
			loc += rem - 1;
		}
		vec.push(xs[loc]);
		xs.remove(loc);
		if xs.len() > 0
		{
			if xs.len() == loc
			{
				loc = 0;
			}
			vec = josephus_recursion(xs, k, loc)
		}
	}
	vec
}

fn josephus<T:Clone+Copy + std::fmt::Debug>(xs:Vec<T>,k:usize)-> Vec<T> {
    
    return (josephus_recursion(xs, k, 0));
    println!("{:?}", xs);
    xs
}