// SPDX-License-Identifier: Apache-2.0

// 283. 移动零

fn main() {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_count = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i - zero_count, i);
            } else {
                zero_count += 1;
            }
        }
    }

    let mut data = vec![0i32, 1, 0, 3, 12];
    move_zeroes(&mut data);
    assert_eq!(data, vec![1i32, 3, 12, 0, 0]);

    let mut data = vec![1i32, 0];
    move_zeroes(&mut data);
    assert_eq!(data, vec![1i32, 0]);

    let mut data = vec![4i32, 2, 4, 0, 0, 3, 0, 5, 1, 0];
    move_zeroes(&mut data);
    assert_eq!(data, vec![4i32, 2, 4, 3, 5, 1, 0, 0, 0, 0]);
    println!("PASS");
}
