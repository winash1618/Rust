fn josephus_recursion<T:Clone+Copy>(xs:Vec<T>, k:usize, l:usize) -> Vec<T> {
    let mut vec = Vec::new();
    if xs.len() >= k
    {
        if loc + k < xs.len() - 1
        {
            loc = loc + k - 1 - xs.len();
        }
        else
        {
            loc = loc + k;
        }
        vec.push(xs[loc]);
        xs.remove(loc);
        if (xs.len() == loc)
            loc = 0;
        vec = josephus_recursion(xs, k, loc);
    }
    else if xs.len() < k && xs.len() > 0
	{
		rem = k % xs.len();
		if loc + rem > xs.len()
		{
			loc = loc + rem - xs.len();
		}
		else
		{
			loc = loc + k
		}
		vec.push(xs[loc]);
		xs.remove(loc);
		if (xs.len() == loc)
			loc = 0;
		vec = josephus_recursion(xs, k, loc)
	}
	vec
}

fn josephus<T:Clone+Copy + std::fmt::Debug>(xs:Vec<T>,k:usize)-> Vec<T> {
    
    return (josephus_recursion(xs, k, 0));
    println!("{:?}", xs);
    xs
}