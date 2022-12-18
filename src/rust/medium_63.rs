/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        // block 1 empty 0
        // 00 到 ij 有路径 dp[i][j] 条
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; n]; m];

        if obstacle_grid[0][0] == 1 || obstacle_grid[m-1][n-1] == 1 {
            return 0;
        }

        for i in 0..m {
            if obstacle_grid[i][0] == 1 {
                break;
            } else {
                dp[i][0] = 1;
            }
        }

        for j in 0..n {
            if obstacle_grid[0][j] == 1 {
                break;
            } else {
                dp[0][j] = 1;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dbg!(dp.clone());
        dp[m - 1][n - 1]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let grids = vec![vec![0, 0]];
    Solution::unique_paths_with_obstacles(grids);
}