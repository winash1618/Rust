// https://www.purplemath.com/modules/factzero.htm - Solution idea got from here.

fn zeros(n: u64) -> u64 {
    let mut num = 5;
    let mut count = 0;
    let cpy = n;
    while n / num > 0
    {
        count += cpy / num;
        num *= 5;
    }
    return count;
}


/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

fn zeros(n: u64) -> u64 {
//     println!("N -> {}", n);
	if n == 0 { 0 } 
	else { n / 5 + zeros(n / 5) }
}


// timed out solution. Not good.


// fn zeros(n: u64) -> u64 {
//     let mut num = 5;
//     let mut k = 0;
//     while num <= n
//     {
//         let j = num;
//         if num % 10 == 0
//         {
//             while num % 10 == 0 && num > 0
//             {
//                 k += 1;
//                 num /= 10;
//             }
//         }
//         if num % 5 == 0
//         {
//             while num % 5 == 0 && num > 0
//             {
//                 k += 1;
//                 num /= 5;
//             }
//         }
//         num = j;
//         num += 5
//     }
//     return k;
// }
