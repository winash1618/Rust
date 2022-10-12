fn make_readable(seconds: u32) -> String {
    let mut time = seconds;
    let mut str = String::new();
    time /= 3600;
    if time == 0
    {
        str.push_str("00");
    }
    else
    {
        if time / 10 == 0
        {
            str.push('0');
        }
        str.push_str(&time.to_string());
    }
    str.push(':');
    time = seconds;
    time %= 3600;
    time /= 60;
    if time == 0
    {
        str.push_str("00");
    }
    else
    {
        if time / 10 == 0
        {
            str.push('0');
        }
        str.push_str(&time.to_string());
    }
    str.push(':');
    time = seconds;
    time %= 3600;
    time %= 60;
    if time == 0
    {
        str.push_str("00");
    }
    else
    {
        if time / 10 == 0
        {
            str.push('0');
        }
        str.push_str(&time.to_string());
    }
    return str;
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// fn make_readable(s: u32) -> String {
//     let m=s/60;
//     let s=s%60;
//     let h=m/60;
//     let m=m%60;
//     format!("{:02}:{:02}:{:02}",h,m,s)    
// }