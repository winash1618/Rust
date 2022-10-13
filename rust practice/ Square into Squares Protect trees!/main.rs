fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut vec : Vec<i64> = Vec::new();
    vec.push(n - 1);
    let mut num = n - 1;
    let mut i = 1;
    while vec.iter().map(|x| x.pow(2)).sum::<i64>() < n * n
    {
        let sum = n * n - vec.iter().map(|x| x.pow(2)).sum::<i64>();
        let sum : i64 = (sum as f64).sqrt() as i64;
        vec.push(sum);
        if i > 2 && vec[i - 1] == vec[i - 2]
        {
            vec.pop();
            vec.pop();
            vec.pop();
            let sum = n * n - vec.iter().map(|x| x.pow(2)).sum::<i64>();
            let sum : i64 = (sum as f64).sqrt() as i64;
            vec.push(sum - 1);
        }
        i += 1;
    }
    let tmp : Vec<i64> = Vec::new();
    if vec.iter().map(|x| x.pow(2)).sum::<i64>() == n * n
    {
        return vec;
    }
    else
    {
        return tmp;
    }
}