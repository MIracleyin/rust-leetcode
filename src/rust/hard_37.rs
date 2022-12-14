/*
 * @lc app=leetcode id=37 lang=rust
 *
 * [37] Sudoku Solver
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtracking(board);
    }

    pub fn backtracking(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] != '.' {
                    continue;
                }
                for k in '1'..='9' {
                    if Self::is_valid(board, i, j, k) {
                        board[i][j] = k;
                        if Self::backtracking(board) {
                            return true;
                        }
                        board[i][j] = '.';
                    }
                }
                return false; // 1..9 can't find right number
            }
        }
        true
    }

    pub fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, val: char) -> bool {
        for i in 0..9 {
            if board[row][i] == val {
                return false;
            }
            if board[i][col] == val {
                return false;
            }
        }

        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for i in start_row..(start_row + 3) {
            for j in start_col..(start_col + 3) {
                if board[i][j] == val {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end
