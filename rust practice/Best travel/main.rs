fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
	if ls.len()  < k
		return (-1);
	let mut lst = ls;
	max1 = *lst.iter().min().unwrap();
	lst.remove(lst
		.iter()
		.position(|&x| x == max1)
		.unwrap());
	max2 = *lst.iter().min().unwrap();
	lst.remove(lst
		.iter()
		.position(|&x| x == max2)
		.unwrap());
	let mut tmp = 0;
	for i in &lst {
		// iterate immutably
		let i: &i32 = i; // elements are immutable pointers
		if t - tmp > t - max1 - max2 - i
		{
			tmp = max1 + max2 + i;
		}
	}
	if tmp != 0 && tmp > max1 + max2 + i
	{
		return -1;
	}
	else
	{
		
	}
}