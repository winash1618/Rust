fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut vec : Vec<u64> = Vec::new();
    let mut c = a;
    while c <= b
    {
        let mut r = c;
        let mut k = 0;
        while r > 0
        {
            r /= 10;
            k += 1;
        }
        r = c;
        let mut x = 0;
        while r > 0
        {
            let rem = r % 10;
            x += rem.pow((k).try_into().unwrap());
            k -= 1;
            r /= 10;
        }
        if x == c
        {
            vec.push(c);
        }
        c += 1;
    }
    return vec;
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn is_eureka(n: u64) -> bool {
//     n.to_string()
//         .chars()
//         .enumerate()
//         .map(|(at, char)| u64::pow(char.to_string().parse::<u64>().unwrap(), at as u32 + 1))
//         .reduce(|accum, item| accum + item)
//         == Some(n)
// }

// fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
//     (a..=b)
//         .into_iter()
//         .filter(|e| is_eureka(*e))
//         .collect::<Vec<u64>>()
// }
