// SPDX-License-Identifier: Apache-2.0

// 121. 买卖股票的最佳时机

fn solve(input: &[i32]) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;
    for v in input.iter() {
        if *v < min_price {
            min_price = *v;
        } else if (v - min_price) > max_profit {
            max_profit = v - min_price;
        }
    }
    return max_profit;
}

fn main() {
    assert_eq!(solve(&[7i32, 1, 5, 3, 6, 4]), 5);

    println!("PASS!");
}
