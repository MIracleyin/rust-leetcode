/*
 * @lc app=leetcode id=216 lang=rust
 *
 * [216] Combination Sum III
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Self::backtracking2(&mut result, &mut path, n, k, 0, 1);
        result
    }

    fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        target_sum: i32,
        k: i32,
        mut sum: i32,
        start_index: i32,
    ) {
        let len = path.len() as i32;
        if len == k {
            if sum == target_sum {
                result.push(path.to_vec());
            }
            return;
        }
        for i in start_index..=9 {
            sum += i;
            path.push(i);
            Self::backtracking(result, path, target_sum, k, sum, i + 1);
            sum -= i;
            path.pop();
        }
    }

    pub fn backtracking2(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        target_sum: i32,
        k: i32,
        mut sum: i32,
        start_index: i32,
    ) {
        let len = path.len() as i32;
        if len == k {
            if sum == target_sum {
                result.push(path.to_vec());
            }
            return;
        }
        for i in start_index..=9 {
            sum += i;
            path.push(i);
            Self::backtracking2(result, path, target_sum, k, sum, i+1); // using i rather than srart_index
            sum -= i;
            path.pop();
        }
    }

    pub fn backtracking1(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        target_num: i32,
        k: i32,
        mut sum: i32,
        start_idx: i32,
    ) {
        let len = path.len() as i32;
        if len == k {
            if sum == target_num {
                result.push(path.to_vec());
            }
            return;
        }

        for i in start_idx..=9 {
            sum += i;
            path.push(i);
            Self::backtracking(result, path, target_num, k, sum, start_idx + 1);
            sum -= i;
            path.pop();
        }
    }
}
// @lc code=end
#[test]
fn it_works() {
    let (n, k) = (3, 7);
    let res = Solution::combination_sum3(n, k);
    dbg!(res);
}
