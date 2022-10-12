fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    let mut count = 0;
    for i in arr {
        // iterate by-value
        if *i != 0
        {
            vec.push(*i);
        }
        else
        {
            count += 1;
        }
    }
    while count > 0
    {
        let num : u8 = 0;
        vec.push(num);
        count -= 1;
    }
    return vec;
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn move_zeros(arr: &[u8]) -> Vec<u8> {
//     let mut i = 0;
//     let mut v = vec![0;arr.len()];
//     for &e in arr {
//         if e!=0 {
//             v[i] = e;
//             i += 1;
//         }
//     }
//     v
// }