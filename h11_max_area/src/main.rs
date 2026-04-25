fn main() {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }
        let mut left_index = 0;
        let mut right_index = height.len() - 1;

        let mut max = 0;

        while left_index < right_index {
            let cur_max =
                std::cmp::min(height[left_index], height[right_index])
                    * (right_index - left_index) as i32;

            if cur_max > max {
                max = cur_max;
            }

            if height[left_index] > height[right_index] {
                right_index -= 1;
            } else {
                left_index += 1;
            }
        }
        max
    }

    assert_eq!(max_area(vec![1i32, 8, 6, 2, 5, 4, 8, 3, 7]), 49);

    println!("PASS");
}
