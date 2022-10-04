fn spin_words(words: &str) -> String {
    let mut string = String::new();
     for word in words.split_whitespace() {
         if word.len() >= 5{
            string.push_str(&reverse_string(word));}
     else{
        string.push_str(word);}
    
         string.push_str(" ");
    }
    string.pop();
    return string;
    
}
fn reverse_string(s: &str) -> String {
  s.chars().rev().collect()
    }