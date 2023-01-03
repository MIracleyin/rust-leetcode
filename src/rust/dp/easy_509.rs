/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn fib_v1(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        println!("{:?}", dp);
        dp[n as usize]
    }

    pub fn fib_v2(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut dp = vec![0; 2];
        dp[0] = 0;
        dp[1] = 1;
        for _ in 2..=n as usize {
            let sum: i32 = dp.iter().sum();
            dp[0] = dp[1];
            dp[1] = sum;
        }
        dp[1]
    }
    
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        Self::fib(n - 1) + Self::fib(n - 2)
    }
}
// @lc code=end
#[test]
fn it_works() {
    let n = 5;
    Solution::fib(n);
}
