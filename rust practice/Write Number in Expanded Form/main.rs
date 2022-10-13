
fn expanded_form(n: u64) -> String {
    let mut str = String::new();
    let mut num = n;
    let mut count : u64 = 0;
    let ten: u64 = 10;
    while num > 0
    {
        let mut rem = num % 10;
        if rem > 0
        {
            let tmp : u64 = ten.pow(count.try_into().unwrap()).try_into().unwrap();
            rem = rem * tmp;
            let string = &mut rem.to_string();
            string.push_str(&str);
            str = string.to_string();
            if num / 10 > 0
            {
                let mut string = String::from(" + ");
                string.push_str(&str);
                str = string.to_string();
            }
        }
             num /= 10;
            count += 1;
    }
    return str;
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn expanded_form(n: u64) -> String {
//     n.to_string()
//         .chars()
//         .enumerate()
//         .filter(|(_, c)| *c != '0')
//         .map(|(i, c)| {
//             ((c as usize - 48) * 10usize.pow(n.to_string().len() as u32 - 1 - i as u32)).to_string()
//         })
//         .collect::<Vec<String>>()
//         .join(" + ")
// }

// Not working on Edge cases

// fn expanded_form(n: u64) -> String {
//     let mut str = String::new();
//     let mut num = n;
//     let mut count = 0;
//     let ten: i32 = 10;
//     while num > 0
//     {
//         let mut rem = num % 10;
//         if rem > 0
//         {
//             let tmp : u64 = ten.pow(count).try_into().unwrap();
//             rem = rem * tmp;
//             let string = &mut rem.to_string();
//             string.push_str(&str);
//             str = string.to_string();
//             if num / 10 > 0
//             {
//                 let mut string = String::from(" + ");
//                 string.push_str(&str);
//                 str = string.to_string();
//             }
//         }
//              num /= 10;
//             count += 1;
//     }
//     return str;
// }