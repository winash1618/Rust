// timed out solution. Not good.
fn zeros(n: u64) -> u64 {
    let mut num = 5;
    let mut k = 0;
    while num <= n
    {
        let j = num;
        if num % 10 == 0
        {
            while num % 10 == 0 && num > 0
            {
                k += 1;
                num /= 10;
            }
        }
        if num % 5 == 0
        {
            while num % 5 == 0 && num > 0
            {
                k += 1;
                num /= 5;
            }
        }
        num = j;
        num += 5
    }
    return k;
}
