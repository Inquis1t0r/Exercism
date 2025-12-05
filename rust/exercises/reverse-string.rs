pub fn reverse(input: &str) -> String {
    let char_vec: Vec<char> = input.chars().collect();
    let reverse_result = char_vec.iter().cloned().rev().collect::<String>();
    dbg!(&reverse_result);
    return reverse_result;
}
