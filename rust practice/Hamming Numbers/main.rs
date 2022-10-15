fn hamming(n: usize) -> u64 {
    if n == 1
	{
		return 1;
	}
	let mut min_value : u64 = 0;
	let mut vec = Vec::new();
	let mut count = 1;
	vec.push(1);
	while count <= n
	{
		min_value = *vec.iter().min().unwrap() as u64;
		let max = u64::MAX;
		if (max / min_value) >= 2 && vec.contains(&(min_value * 2)) == false
		{
			vec.push(min_value * 2);
		}
		if (max / min_value) >= 3 && vec.contains(&(min_value * 3)) == false
		{
			vec.push(min_value * 3);
		}
		if (max / min_value) >= 5 && vec.contains(&(min_value * 5)) == false
		{
			vec.push(min_value * 5);
		}
		vec.remove(vec
			.iter()
			.position(|&x| x == min_value)
			.unwrap());
		count += 1;
	}
	min_value
}

fn main () {
	let ham = hamming(5000);
	println!("{}", ham);
}

/*************************************************************************/
/*************************** Best Solutions *******************************/
/*************************************************************************/

// fn hamming(n: usize) -> u64 {
//     let mut output:Vec<u64> = vec![0; n+1];
//     output[1] = 1;
//     let (mut p2, mut p3, mut p5) = (1,1,1);
//     for i in 2..=n {
//         let (num2, num3, num5) = (output[p2] * 2, output[p3] * 3, output[p5] * 5);
//         output[i] = num2.min(num3).min(num5);
//         if output[i] == num2 {
//             p2 += 1;
//         }  
//         if output[i] == num3 {
//             p3 += 1;
//         }  
//         if output[i] == num5 {
//             p5 += 1;
//         }  
//     }
//     output[n]
// }

// pub fn hamming(n: usize) -> u64 {
// 	use std::collections::BTreeSet;

// 	let mut s: BTreeSet<u64> = BTreeSet::from([1]);
// 	for _ in 1..n {
// 		let x = *s.iter().next().unwrap();
// 		s.remove(&x);
// 		s.insert(x.checked_mul(2).unwrap_or(u64::MAX));
// 		s.insert(x.checked_mul(3).unwrap_or(u64::MAX));
// 		s.insert(x.checked_mul(5).unwrap_or(u64::MAX));
// 	}
// 	*s.iter().next().unwrap()
// }

// use std::{collections::BTreeSet};

// fn next_hamming(hamming: &mut BTreeSet<u64>, n: u64) {
//     if !hamming.contains(&n) {
//         hamming.insert(n);
//         if n < u64::MAX / 2 {
//             next_hamming(hamming, n * 2);
//         }
//         if n < u64::MAX / 3 {
//             next_hamming(hamming, n * 3);
//         }
//         if n < u64::MAX / 5 {
//             next_hamming(hamming, n * 5);
//         }
//     }
// }

// fn hamming(n: usize) -> u64 {
//     let mut hamming: BTreeSet<u64> = BTreeSet::new();
//     next_hamming(&mut hamming, 1);
//     *hamming.iter().nth(n - 1).unwrap()
// }