// SPDX-License-Identifier: Apache-2.0

// 128. 最长连续序列

fn main() {
    fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let hashed: HashSet<i32> = nums.into_iter().collect();

        let mut results: HashSet<i32> = HashSet::new();
        for v in hashed.iter() {
            let mut cur_v = *v;
            let mut count = 1;
            if !hashed.contains(&(cur_v - 1)) {
                while hashed.contains(&(cur_v + 1)) {
                    count += 1;
                    cur_v += 1;
                }
            }
            results.insert(count);
        }

        results.drain().max().unwrap_or(0)
    }

    assert_eq!(longest_consecutive(vec![100i32, 4, 200, 1, 3, 2]), 4);
    println!("PASS")
}
