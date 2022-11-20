
/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] 螺旋矩阵 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let (mut startX, mut startY) = (0, 0);
        let mut lp = n / 2;
        let mid = n / 2;
        let mut count = 1;
        let mut offset = 1;
        let (mut i, mut j ) = (0, 0);

        while lp > 0 {
            i = startX;
            j = startY;

            // x keep, iter y
            while j < (startY + n as usize - offset) {
                matrix[i][j] = count;
                count += 1;
                j += 1;
            }

            // iter x, y keep
            while i < (startY + n as usize - offset) {
                matrix[i][j] = count;
                count += 1;
                i += 1;
            }

            // iter y, x keep
            while j > startY  {
                matrix[i][j] = count;
                count += 1;
                j -= 1;                
            }

            while i > startX {
                matrix[i][j] = count;
                count += 1;
                i -= 1;
            }

            startX += 1;
            startY += 1;
            offset += 2;
            lp -= 1;
        }

        if n % 2 != 0 {
            matrix[mid as usize][mid as usize] = n * n;
        }
        
        matrix
    }
}
// @lc code=end

