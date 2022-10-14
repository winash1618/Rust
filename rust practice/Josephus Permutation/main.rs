fn josephus_recursion<T:Clone+Copy + std::fmt::Debug>(xs:Vec<T>, k:usize, loc:usize) -> Vec<T> {
    let mut vec = Vec::new();
	let mut rem = k;
    let mut loc = loc;
    let mut xs = xs;
    
    if xs.len() > 0
	{
		if xs.len() < k
		{
			rem = k % xs.len();
		}
		if loc == 0 && rem == 0
		{
			loc = xs.len() - 1;
		}
		else if (loc + rem) > xs.len()
		{
			loc = (loc + rem) - xs.len() - 1;
		}
		else
		{
			loc = loc + rem - 1;
		}
		vec.push(xs[loc]);
		xs.remove(loc);
		if xs.len() > 0
		{
			if xs.len() == loc
			{
				loc = 0;
			}
			vec.extend(josephus_recursion(xs, k, loc));
		}
	}
	vec
}

fn josephus<T:Clone+Copy + std::fmt::Debug>(xs:Vec<T>,k:usize)-> Vec<T> {
	let v = josephus_recursion(xs, k, 0);
	v
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn josephus<T: std::fmt::Debug>(mut live: Vec<T>, k: usize) -> Vec<T> {
//     let mut dead = Vec::new();
//     let mut i = (k - 1) % live.len();
//     loop {
//         dead.push(live.remove(i));
//         if live.is_empty() {
//             break;
//         }
//         i = (i + k - 1) % live.len();
//     }
// 	// println!("{:?}", dead);
//     dead
// }

// fn main()
// {
// 	josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1);
// 	josephus("CodeWars".chars().collect::<Vec<char>>(), 4);
// }