/*
 * @lc app=leetcode id=455 lang=rust
 *
 * [455] Assign Cookies
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_content_children_v1(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        (g.sort(), s.sort());
        let mut c = 0;
        for i in 0..s.len() {
            if c < g.len() && g[c] <= s[i] { // 
                c += 1;
            }
        }
        c as i32
    }

    pub fn find_content_children_v2(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g; // children
        let mut s = s; // cookie
        (g.sort(), s.sort());
        let (mut child, mut cookie) = (0usize, 0usize);
        while child < g.len() && cookie < s.len() {
            if g[child] <= s[cookie] {
                child += 1;
            }
            cookie += 1;
        }
        child as i32
    }

    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g; // children
        let mut s = s; // cookie
        (g.sort(), s.sort());
        let mut res = 0;
        if s.len() == 0 {
            return 0;
        } 
        let mut idx = s.len() as i32 - 1;
        for i in (0..g.len()).rev() {
            if idx >= 0 && s[idx as usize] >= g[i] {
                res += 1;
                idx -= 1;
            }
        }
        res
    }
}
// @lc code=end

#[test]
fn it_works() {
    let g = vec![1,2,3];
    let s = vec![3];
    Solution::find_content_children(g, s);
}