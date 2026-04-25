// SPDX-License-Identifier: Apache-2.0

// 15. 三数之和

fn main() {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::{collections::HashMap, ops::Neg};

        fn two_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
            // <number, index>
            let mut hashed: HashMap<i32, usize> = HashMap::new();

            let mut ret: Vec<(i32, i32)> = Vec::new();

            let mut dup_pushed = false;
            for index in 0..nums.len() {
                let v = nums[index];
                if v * 2 == target && hashed.contains_key(&v) {
                    if !dup_pushed {
                        dup_pushed = true;
                        ret.push((v, v));
                    }
                } else {
                    hashed.insert(v, index);
                }
            }

            for index in hashed.values().map(|v| *v) {
                let v = nums[index];
                let remains = target - v;
                if let Some(other_index) = hashed.get(&remains)
                    && *other_index != index
                {
                    let other = nums[*other_index];
                    if v >= other {
                        ret.push((other, v));
                    }
                }
            }
            ret
        }

        let mut ret: Vec<Vec<i32>> = Vec::new();

        if nums.len() < 3 {
            return ret;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut pre_v = nums[nums.len() - 1];
        for i in 0..(nums.len() - 2) {
            if i != 0 && nums[i] == pre_v {
                continue;
            } else {
                pre_v = nums[i];
            }
            let two_sums = two_sum(&nums[(i + 1)..], nums[i].neg());

            for (b, c) in two_sums {
                ret.push(vec![nums[i], b, c]);
            }
        }
        ret
    }

    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1i32, 0, 1], vec![-1i32, -1, 2]]
    );
    assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0i32, 0, 0]]);
    println!("PASS");
}
