/*
 * @lc app=leetcode id=51 lang=rust
 *
 * [51] N-Queens
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut chessboard: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
        Self::backtracking(&mut result, &mut chessboard, n as usize, 0);
        result
    }

    pub fn backtracking(
        result: &mut Vec<Vec<String>>,
        chessboard: &mut Vec<Vec<char>>,
        n: usize,
        row: usize,
    ) {
        if row == n {
            // backtrcking layer
            let mut chessboard_clone: Vec<String> = Vec::new();
            for i in chessboard {
                chessboard_clone.push(i.iter().collect::<String>());
            }
            result.push(chessboard_clone);
            return;
        }

        for col in 0..n {
            if Self::is_valid(row, col, chessboard, n) {
                chessboard[row][col] = 'Q';
                Self::backtracking(result, chessboard, n, row + 1);
                chessboard[row][col] = '.';
            }
        }
    }

    pub fn is_valid(row: usize, col: usize, chessboard: &mut Vec<Vec<char>>, n: usize) -> bool {
        for i in 0..row {
            if chessboard[i][col] == 'Q' {
                return false;
            }
        }

        for (i, j) in ((0..row).rev()).zip((0..col).rev()) { // from Q loc to 45 below
            if chessboard[i][j] == 'Q' {
                return false;
            }
        }

        for (i, j) in ((0..row).rev()).zip(col + 1..n) {
            if chessboard[i][j] == 'Q' {
                return false;
            }
        }
        true
    }
}
// @lc code=end
#[test]
fn it_works() {
    let row = 1;
    let col = 1;
    let n = 4;
    // for i in (0..1).rev() {
    //     println!("{:?}", i);
    // }
    // for (i, j) in ((0..row).rev()).zip((0..col).rev()) {
    //     println!("{:?} {:?}", i, j);
    // }
    let l0 = vec!['.', '.', 'Q', '.'];
    let l1 = vec!['.', 'Q', '.', '.'];
    let l2 = vec!['.', '.', '.', '.'];
    let l3 = vec!['.', '.', '.', '.'];
    let mut c = vec![l0, l1, l2, l3];
    // Solution::solve_n_queens(n);
    assert_eq!(Solution::is_valid(row, col, &mut c, n), false);
}
