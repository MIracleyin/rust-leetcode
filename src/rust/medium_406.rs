/*
 * @lc app=leetcode id=406 lang=rust
 *
 * [406] Queue Reconstruction by Height
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by(|a, b| {
            if a[0] == b[0] { return a[1].cmp(&b[1]); } 
            b[0].cmp(&a[0])
        });
        let mut que: Vec<Vec<i32>> = Vec::new();
        que.push(people[0].clone());
        for i in 1..people.len() {
            let position = people[i][1];
            que.insert(position as usize, people[i].clone());
        }
        que
    }
}
// @lc code=end
