// SPDX-License-Identifier: Apache-2.0

// 49. 字母异位词分组

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut hashed: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for val in strs {
        let mut sorted_val = val.chars().collect::<Vec<char>>();
        sorted_val.sort_unstable();
        hashed
            .entry(sorted_val)
            .and_modify(|v| v.push(val.clone()))
            .or_insert(vec![val]);
    }
    hashed.drain().map(|(_, v)| v).collect()
}

fn main() {
    assert_eq!(
        group_anagrams(vec![
            "eat".to_string(),
            "tea".into(),
            "tan".into(),
            "ate".into(),
            "nat".into(),
            "bat".into()
        ]),
        vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
        ]
    );

    println!("Hello, world!");
}
