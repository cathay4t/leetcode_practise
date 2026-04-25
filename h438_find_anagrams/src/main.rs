// SPDX-License-Identifier: Apache-2.0

// 438. 找到字符串中所有字母异位词

fn main() {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut content = vec![0i32; 26];
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut ret: Vec<i32> = Vec::new();

        if s.len() < p.len() {
            return ret;
        }

        for i in 0..p.len() {
            content[(s[i] - 'a' as u8) as usize] += 1;
            content[(p[i] - 'a' as u8) as usize] -= 1;
        }
        if content.iter().all(|c| *c == 0) {
            ret.push(0);
        }

        for i in 0..(s.len() - p.len()) {
            content[(s[i] - 'a' as u8) as usize] -= 1;
            content[(s[i + p.len()] - 'a' as u8) as usize] += 1;
            if content.iter().all(|c| *c == 0) {
                ret.push((i + 1) as i32);
            }
        }
        ret
    }

    assert_eq!(
        find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0i32, 6]
    );

    println!("PASS!");
}
