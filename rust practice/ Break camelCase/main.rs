fn solution(s: &str) -> String {
    let mut str = String::new();
    for c in s.chars()
    {
        if c >= 'A' && c <= 'Z'
        {
            str.push(' ');
        }
        str.push(c);
    }
    return str;
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn solution(s: &str) -> String {
//     let mut res = String::new();
//     for c in s.chars() {
//         if c.is_uppercase() {
//             res.push(' ');
//         }
//         res.push(c);
//     }
//     res
// }
