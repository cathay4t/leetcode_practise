// SPDX-License-Identifier: Apache-2.0

// 42. 接雨水

fn main() {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max = vec![0i32; height.len()];

        if height.len() < 1 {
            return 0;
        }

        for i in 0..height.len() {
            if i == 0 {
                left_max[0] = height[0];
            } else {
                left_max[i] = if height[i] > left_max[i - 1] {
                    height[i]
                } else {
                    left_max[i - 1]
                };
            }
        }
        let mut right_max = vec![0i32; height.len()];
        for i in 0..height.len() {
            let i = height.len() - 1 - i;
            if i == height.len() - 1 {
                right_max[i] = height[height.len() - 1];
            } else {
                right_max[i] = if height[i] > right_max[i + 1] {
                    height[i]
                } else {
                    right_max[i + 1]
                };
            }
        }

        let mut sum = 0;

        for i in 0..height.len() {
            sum += std::cmp::min(left_max[i], right_max[i]) - height[i];
        }

        sum
    }

    assert_eq!(trap(vec![0i32, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    println!("PASS");
}
