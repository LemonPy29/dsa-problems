fn len_(p: (usize, usize)) -> usize {
    p.1 - p.0
}

fn palindrome_at(s: &String, i: usize, l: i32) -> (usize, usize) {
    let mut j = 1;
    let _i = i as i32;
    let bytes = s.as_bytes();

    loop {
       
        if (_i - j + l < 0) | (_i + j >= s.len() as i32) {
            break;
        }

        let prev_byte = bytes[(_i-j+l) as usize];
        let next_byte = bytes[(_i+j) as usize];
        if prev_byte == next_byte { 
            j += 1;
        } else {
            break;
        } 
    }
    ((_i - j + l + 1) as usize, (_i + j) as usize)
}

fn longest_palindrome(s: String) -> String {
    let mut idxs = (0, 1);
    for i in 0..s.len() {
        let odd = palindrome_at(&s, i, 0);
        let even = palindrome_at(&s, i, 1);
        let winner = if len_(odd) > len_(even) { odd } else { even };
        idxs = if len_(winner) > len_(idxs) { winner } else { idxs };
    }
    String::from(&s[idxs.0..idxs.1])
}

fn main() {
    let s = String::from("bbbbaaa");
    println!("{}", longest_palindrome(s));
}
