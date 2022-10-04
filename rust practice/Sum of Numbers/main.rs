fn get_sum(a: i64, b: i64) -> i64
{
    if a > b
   { let size = a - b + 1;
    
    return size * (a + b ) / 2;
       
       }
    else
    {
        let size = b - a + 1;
    
    return size * (b + a) / 2;
    }
}