// SPDX-License-Identifier: Apache-2.0

// 15. 三数之和

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;

        if nums.len() < 3 {
            return ret;
        }

        nums.sort();

        for i in 0..nums.len() {
            let num_i = nums[i];
            // first(smallest) number should always be less than zero
            if num_i > 0 {
                break;
            }
            // skip if the same number has checked before.
            if i > 0 && nums[i - 1] == num_i {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let num_j = nums[j];
                let num_k = nums[k];
                let sum = num_i + num_j + num_k;
                if sum < 0 {
                    j += 1;
                } else if sum > 0 {
                    k -= 1;
                } else {
                    ret.push(vec![num_i, num_j, num_k]);
                    j += 1;
                    k -= 1;
                    // skip same number
                    while j < k && nums[j] == num_j {
                        j += 1;
                    }
                    // skip same number
                    while j < k && nums[k] == num_k {
                        k -= 1;
                    }
                }
            }
        }
        ret
    }
}

struct Solution;

fn main() {
    let mut ret = Solution::three_sum(vec![-1, 0, 1, 1, 2, -1, -4]);
    ret.sort_unstable();

    assert_eq!(ret, vec![vec![-1i32, -1, 2], vec![-1i32, 0, 1]]);
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0i32, 0, 0]]);
    println!("PASS");
}
