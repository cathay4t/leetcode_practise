// SPDX-License-Identifier: Apache-2.0

use rand::RngExt;

impl Solution {
    // include right
    fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
        let pivot = rand::rng().random_range(left..=right);

        let num_pivot = nums[pivot];

        nums.swap(pivot, left);

        let mut i = left + 1;
        let mut j = right;

        loop {
            while i <= j && nums[i] < num_pivot {
                i += 1;
            }
            // now nums[i] >= nums[pivot]

            while i <= j && nums[j] > num_pivot {
                j -= 1;
            }

            if i >= j {
                break;
            }
            // now nums[j] < nums[pivot]
            // so we swap i, j
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
        nums.swap(left, j);
        j
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target_index = nums.len() - k as usize;
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut pivot = Self::partition(&mut nums, left, right);
        while target_index != pivot {
            if pivot > target_index {
                // in zone of [left, pivot - 1]
                right = pivot - 1;
            } else {
                // in zone of [pivot + 1, right]
                left = pivot + 1;
            }
            pivot = Self::partition(&mut nums, left, right);
        }
        nums[pivot]
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_kth_largest(vec![3i32, 2, 1, 5, 6, 4], 2),
        5i32
    );

    assert_eq!(
        Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
        4
    );
    println!("PASS");
}
