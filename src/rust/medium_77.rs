use std::path;

/*
 * @lc app=leetcode id=77 lang=rust
 *
 * [77] Combinations
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new(); // save res set
        let mut path: Vec<i32> = Vec::new(); // save one res
        Self::backtracking(&mut result, &mut path, n, k, 1);
        result
    }

    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        n: i32,
        k: i32,
        start_idx: i32,
    ) {
        if path.len() == k as usize {
            result.push(path.to_vec());
            return;
        }

        for i in start_idx..=n {
            path.push(i);
            println!("idx: {:?}, result: {:?}, path: {:?}", i, result, path);
            Self::backtracking(result, path, n, k, i + 1);
            path.pop();
            println!("idx: {:?}, result: {:?}, path: {:?}", i, result, path);
        }
    }

    pub fn backtracking_v1(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        n: i32,
        k: i32,
        start_idx: i32,
    ) {
        if path.len() == k as usize {
            result.push(path.to_vec());
            return;
        }

        for i in start_idx..=n {
            //
            path.push(i);
            Self::backtracking_v1(result, path, n, k, i + 1);
            path.pop();
        }
    }

    pub fn backtracking_(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        n: i32,
        k: i32,
        start_idx: i32,
    ) {
        if path.len() == k as usize {
            result.push(path.to_vec());
            return;
        }
        for i in start_idx..=n - (k - path.len() as i32) + 1 {
            path.push(i);
            Self::backtracking(result, path, n, k, i + 1);
            path.pop();
        }
    }
}
// @lc code=end

#[test]
fn it_works() {
    let (n, k) = (4, 2);
    let res = Solution::combine(n, k);
    dbg!(res);
}
