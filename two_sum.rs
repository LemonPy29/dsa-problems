use::std::collections::HashMap;

fn two_sum_brute_force(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = Vec::new();
    let n = nums.len();
    for i in 0..n {
        for j in i..n {
            if &nums[i] + &nums[j] == target {
                solution.push(i as i32);
                solution.push(j as i32);
                break;
            }
        }
    }
    solution
}

fn two_sum_hash_map(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = Vec::new();

    let mut comp = HashMap::new();
    for (i, val) in nums.iter().enumerate() {
        if let Some(j) = comp.get(&(target - val)) {
            solution.push(*j as i32);
            solution.push(i as i32);
            break;
        } else {
            comp.insert(val, i);
        }
    }
    solution
}

fn main() {
    let test = vec![ 2, 7, 11, 15 ];
    println!("{:?}", two_sum_brute_force(&test, 9));
    println!("{:?}", two_sum_hash_map(&test, 9));
}
