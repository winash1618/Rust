
fn digital_root(n: i64) -> i64 {
    let mut num = n;
    let mut sum : i64= 0;

    while num > 0
    {
        sum += num % 10;
        if sum >= 10
        {
            sum = sum % 10 + sum / 10;
        }
        num /= 10;
    }
  
    return sum;
}


/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn digital_root(n: i64) -> i64 {
//     if n/10==0 {n} else {digital_root(n%10 + n/10)}
// }

// timed out solution. Not good.

// fn digital_root(n: i64) -> i64 {
//     let mut num = n;
//     let mut sum : i64= 0;
//     while num / 10 > 0
//     {
//         while num > 0
//         {
//             sum += num % 10;
//             num /= 10;
//         }
//         num = sum;
//     }
//     return num;
// }