// SPDX-License-Identifier: Apache-2.0

fn main() {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        // contains <pre_sum, count>
        let mut pre_sums: HashMap<i32, i32> = HashMap::new();

        let mut pre_sum = 0;
        for i in 0..nums.len() {
            pre_sum += nums[i];
            pre_sums.entry(pre_sum).and_modify(|i| *i +=1).or_insert(1);
        }

        println!("HAHA685 {:?}",  pre_sums);

        0

    }

    assert_eq!(subarray_sum(vec![1i32, 1, 1], 2), 2);
    println!("PASS");
}
