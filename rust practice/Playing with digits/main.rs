fn dig_pow(n: i64, p: i32) -> i64 {
    let mut x = n;
    let mut k = 0;
    while x > 0
    {
        k += 1;
        x /= 10;
    }
    let mut temp = n;
    x = 0;
    while temp > 0
    {
        let r : i64 = temp % 10;
        k -= 1;
        x += r.pow((p + k).try_into().unwrap());
        temp /= 10;
    }
    if x % n == 0
    {
        return x / n;
    }
    else
    {
        return -1;
    }
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn dig_pow(n: i64, p: i32) -> i64 {
//     let r: i64 = n.to_string().chars()
//     .map(|c| (c as i64) - 48)
//     .enumerate()
//     .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
//     .sum();
    
//     match r%n {
//         0 => r/n,
//         _ => -1,
//     }
// }