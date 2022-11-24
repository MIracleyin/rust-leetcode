/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_element_v1(nums: &mut Vec<i32>, val: i32) -> i32 {
        // v 
        let mut size = nums.len();
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                for j in i+1..nums.len() {
                    nums[j - 1] = nums[j];
                }
                size -= 1;
                i -= 1;
            }
            i += 1;
        }
        size as i32
    }

    pub fn remove_element_v2(nums: &mut Vec<i32>, val: i32) -> i32 {
        // v 
        let mut size = nums.len();
        let mut fast = 0;
        let mut slow = 0;
        while fast < nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            } else {
                size -= 1;
            }
            fast += 1;
        }
        size as i32
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // v 
        let mut slow = 0;
        for pos in (0..nums.len()) {
            if nums[pos] != val {
                nums[slow] = nums[pos];
                slow += 1;
            }
        }
        slow as i32
    }
}
// @lc code=end

mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut n = vec![-1,0,3,5,5,9,12];
        let val = 5;
        let size = Solution::remove_element(&mut n, val);
        println!("{:?}", n);
        println!("{}", size);
    }
}