// SPDX-License-Identifier: Apache-2.0

// 3. 无重复字符的最长子串

fn main() {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        let chars: Vec<char> = s.chars().collect();

        let mut right_index = -1i32;
        let mut char_set: HashSet<char> = HashSet::new();
        let mut max = 0i32;

        for left_index in 0..chars.len() {
            if left_index > 0 {
                char_set.remove(&chars[left_index - 1]);
            }
            while ((right_index + 1) as usize) < chars.len()
                && !char_set.contains(&chars[(right_index + 1) as usize])
            {
                right_index += 1;
                char_set.insert(chars[right_index as usize]);
            }
            max = std::cmp::max(
                ((right_index + 1) as usize - left_index) as i32,
                max,
            );
        }
        max
    }

    assert_eq!(length_of_longest_substring("abcabcbb".into()), 3i32);

    println!("PASS!");
}
