fn count_duplicates(text: &str) -> u32
{    
    let mut arr:[u32;128] = [0;128]; // take the ascii range that contains alpha numerical character.
    for i in 0..arr.len()
    {
        arr[i] = 0;
    }
    for c in text.chars()
    {
        if c >= 'A' && c <= 'Z'
        {
            arr[(c as usize) + 32]  += 1; // if the letter is capital change it to small index then add one
        } 
        else
        {
            arr[c as usize] += 1; // other wise just add one 
        }
        
    }
    let mut k = 0;
    for i in 0..arr.len()
    {
        if arr[i] >= 2 // loop throught the array and if count is >= 2 means presence of duplicate.
        {
            k += 1;
        }
    }
    return k;
}

/*************************************************************************/
/*************************** Best Solution *******************************/
/*************************************************************************/

// use std::collections::HashMap;

// fn count_duplicates(text: &str) -> u32 {
//     let mut char_count: HashMap<char, u32> = HashMap::new();
//     for c in text.to_lowercase().chars() {
//         let mut e = char_count.entry(c).or_default();
//         *e += 1;
//     }
//     char_count.values().filter(|&&v| v > 1).count() as u32
// }