// Working but timed out
fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut vec : Vec<i64> = Vec::new();
    vec.push(n - 1);
    while vec.iter().map(|x| x.pow(2)).sum::<i64>() < n * n
    {
        let sum = n * n - vec.iter().map(|x| x.pow(2)).sum::<i64>();
        let sum : i64 = (sum as f64).sqrt() as i64;
		if vec[vec.len() - 1] > sum
		{
			vec.push(sum);
		}
		else
		{
			if vec.len() > 1
			{
				let tmp = vec[vec.len() - 2];
				println!("ls {} {}", vec[vec.len() - 1], vec[vec.len() - 2]);
				vec.pop();
				vec.pop();
				// let sum = n * n - vec.iter().map(|x| x.pow(2)).sum::<i64>();
				// let sum : i64 = (sum as f64).sqrt() as i64;
				// println!("sum {} and length {}", sum, vec.len());
				vec.push(tmp - 1);
			}
		}
        
    }
    if vec.iter().map(|x| x.pow(2)).sum::<i64>() == n * n
    {
        let mut tmp : Vec<i64> = Vec::new();
        while vec.len() != 0
        {
			println!("{} {}", vec[vec.len() - 1], vec.len() - 1);
            tmp.push(vec[vec.len() - 1]);
            vec.pop();
        }
        Some(tmp)
    }
    else
    {
        None
    }
}
fn main()
{
	decompose(50);
}
// Not Working
// fn decompose(n: i64) -> Option<Vec<i64>> {
//     let mut vec : Vec<i64> = Vec::new();
//     vec.push(n - 1);
//     let mut num = n - 1;
//     let mut i = 1;
//     while vec.iter().map(|x| x.pow(2)).sum::<i64>() < n * n
//     {
//         let sum = n * n - vec.iter().map(|x| x.pow(2)).sum::<i64>();
//         let sum : i64 = (sum as f64).sqrt() as i64;
//         vec.push(sum);
//         if i > 2 && vec[i - 1] == vec[i - 2]
//         {
//             vec.pop();
//             vec.pop();
//             vec.pop();
//             let sum = n * n - vec.iter().map(|x| x.pow(2)).sum::<i64>();
//             let sum : i64 = (sum as f64).sqrt() as i64;
//             vec.push(sum - 1);
//         }
//         i += 1;
//     }
//     let tmp : Vec<i64> = Vec::new();
//     if vec.iter().map(|x| x.pow(2)).sum::<i64>() == n * n
//     {
//         return vec;
//     }
//     else
//     {
//         return tmp;
//     }
// }