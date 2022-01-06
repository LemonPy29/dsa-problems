use std::cmp::max;
use std::collections::HashMap;

fn length_of_longest_substring(s: String) -> i32 {
    let mut longest: i32 = 0;
    let mut current_longest: i32 = 0;
    let mut n_char: i32 = 0;
    let mut char_indices = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        if let Some(idx) = char_indices.get(&c) {
            if *idx as i32 >= n_char {
                current_longest -= (*idx as i32) - n_char;
                n_char = (*idx as i32) + 1;
            } else {
                current_longest += 1;
            }
        } else {
            current_longest += 1;
        }
        char_indices.insert(c, i);
        longest = max(longest, current_longest);
    }
    longest
}

fn main() {
    let test = String::from("bbb");
    println!("{:?}", length_of_longest_substring(test));
}
