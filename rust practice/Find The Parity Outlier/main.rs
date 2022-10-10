fn find_outlier(values: &[i32]) -> i32 {
    let mut k = 0;
    let mut j = 0;
    let mut l = 0;
    let mut r = 0;
    for i in 0..values.len()
    {
        if values[i] % 2 == 0
        {
            l = values[i];
            k = k + 1;
            if j >= 2
            {
                break;
            }
        }
        if values[i] % 2 != 0
        {
            r = values[i];
            j = j + 1;
            if k >= 2
            {    
                 break;
            }
        }
    }
    if k >= 2
       { return r;}
    else
       {return l;}
}