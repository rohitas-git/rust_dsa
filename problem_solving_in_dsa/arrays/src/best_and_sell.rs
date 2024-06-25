/* --------------------- Best Time to Buy and Sell Stock --------------------- */

// what if we buy today, what's the following highest price?
// time - O( n^2 <= (n-1)*n/2 )      space - O(1)
fn buy_today_soln(arr: &[u32]) -> u32 {
    let mut max_profit = 0;

    for (i, buying_price) in arr[..arr.len() - 1].iter().enumerate() {
        let mut highest_following_price = 0;

        for val in arr.iter().skip(i) {
            highest_following_price = u32::max(highest_following_price, *val);
        }
        let curr_profit = highest_following_price - buying_price;
        max_profit = u32::max(max_profit, curr_profit);
    }
    max_profit
}

// can we do it faster?
// could a different perspective help?
// what if we sell today, what's the previous highest price?
// time - O(n)      space - O(1)
fn sell_today(arr: &[u32]) -> u32 {
    let mut max_profit = 0;

    let mut lowest_preceding_price = arr[0];
    for selling_price in arr[1..].iter() {
        if lowest_preceding_price > *selling_price {
            lowest_preceding_price = *selling_price;
        } else {
            let curr_profit = selling_price - lowest_preceding_price;
            max_profit = u32::max(max_profit, curr_profit);
        }
    }
    max_profit
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_buy_today_sol() {
        let arr = [7, 1, 5, 3, 6, 4];
        assert_eq!(buy_today_soln(&arr), 5);
    }
}
