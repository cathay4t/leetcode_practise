// SPDX-License-Identifier: Apache-2.0

// 1. 两数之和

fn find_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut hashed: HashMap<i32, i32> = HashMap::new();

    for (index, value) in nums.into_iter().enumerate() {
        if value * 2 == target
            && let Some(pre_index) = hashed.get(&value)
        {
            return vec![*pre_index, index as i32];
        } else {
            hashed.insert(value, index as i32);
        }
    }

    for (value1, index1) in hashed.iter() {
        // TODO: handle overflow
        if let Some(index2) = hashed.get(&(target - value1))
            && index2 != index1
        {
            if index1 > index2 {
                return vec![*index2, *index1];
            } else {
                return vec![*index1, *index2];
            }
        }
    }
    Vec::new()
}

fn main() {
    let input = vec![2i32, 7, 11, 15];

    assert_eq!(find_sum(input, 9), vec![0i32, 1]);

    assert_eq!(find_sum(vec![3, 3], 6), vec![0i32, 1]);
    println!("PASS");
}
