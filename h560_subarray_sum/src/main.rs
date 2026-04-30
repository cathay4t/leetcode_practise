// SPDX-License-Identifier: Apache-2.0

fn main() {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        // we need to find nums[i, j] (both inclusive) subarray with sum of k.
        // define `pre_sums[i]` means the sum of numbers before(include) i.
        // So k = pre_sums[j] - pre_sums[i - 1]
        // For each `pre_sums[j]`, we need to find out how many count
        // of sum equal to `presum[i-1] - k`

        // `HashMap<pre_sum, count>`
        let mut pre_sums = HashMap::new();
        // empty array sums 0, so it always has 1 count
        pre_sums.insert(0, 1);

        let mut pre_sum = 0;
        let mut count = 0;

        for j in 0..nums.len() {
            pre_sum += nums[j];

            if let Some(c) = pre_sums.get(&(pre_sum - k)) {
                count += *c;
            }
            *(pre_sums.entry(pre_sum).or_insert(0)) += 1;
        }
        count
    }

    assert_eq!(subarray_sum(vec![1i32, 1, 1], 2), 2);
    println!("PASS");
}
