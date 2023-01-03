/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn unique_paths_v1(m: i32, n: i32) -> i32 {
        Solution::dfs(1, 1, m, n)
    }
    pub fn dfs(i: i32, j: i32, m: i32, n: i32) -> i32 {
        if i > m || j > n {
            return 0;
        }
        if i == m && j == n {
            return 1;
        }
        Solution::dfs(i + 1, j, m, n) + Solution::dfs(i, j + 1, m, n)
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // 00 到 ij 有路径 dp[i][j] 条
        // dp[i][j] = dp[i-1][j]  dp[i][j-1]
        // dp[i][j] 可能有两种情况，dp[i-1][j]+1 dp[i][j-1]+1
        // dp[i][j] = dp[i-1][j] + dp[i][j-1]
        let m = m as usize;
        let n = n as usize; 
        let mut dp = vec![vec![0; n]; m]; // not m n

        for i in 0..m {
            dp[i][0] = 1;
        }

        for j in 1..n {
            dp[0][j] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
        dbg!(dp.clone());
        dp[m - 1][n - 1]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let (m, n) = (3, 7);
    let res = Solution::unique_paths(m, n);
    dbg!(res);
}