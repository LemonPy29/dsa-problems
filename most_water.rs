use::std::cmp::{min, max};

fn most_water(height: Vec<i32>) -> i32 {
    let mut iter = height.iter();
    let mut start = iter.next();
    let mut end = iter.next_back();
    let mut large = (height.len() as i32) - 1;
    let mut water = 0;

    while let (Some(s), Some(e)) = (start, end) {
        water = max(water, min(s, e) * large);
        if e > s {
            start = iter.next();
        } else {
            end = iter.next_back();
        }
        large -= 1;
    }

    water
}

fn main() {
    let test = vec![2, 3, 4, 5, 18, 17, 6];
    println!("{:?}", most_water(test));
}
