fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let l = s.len() as i32;
    let m = 2 * num_rows - 2;

    let mut result = vec![];
    let bytes = s.as_bytes();

    for j in 0..num_rows {
        for i in (j..l).step_by(m as usize) {
            result.push(bytes[i as usize]);
            if j != 0 && j != num_rows - 1 && m - 2 * j + i < l {
                result.push(bytes[(m - 2 * j + i) as usize]);
            }
        }
    }

    String::from_utf8(result).unwrap()
}

fn main() {
    let s = String::from("ABCDEF");
    let n = 4;
    println!("{:?}", convert(s, n));
}
