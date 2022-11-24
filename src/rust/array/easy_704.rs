/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */
struct Solution;
// @lc code=start
impl Solution {
    // [a, b]
    pub fn search_v1(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l as i32 <= r as i32 {
            let mid = (l + r) / 2; 
            println!("{}", mid);
            if nums[mid] > target {
                r = mid - 1;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                return mid as i32;
            }
        }
        -1

    }
    // [)
    pub fn search_v2(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = (l + r) / 2; 
            println!("{}", mid);
            if nums[mid] > target {
                r = mid;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                return mid as i32;
            }
        }
        -1
    }
    // (-1,0,3,5,9,12] 13
    // 
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l as i32 <= r as i32 {
            let mid = (l + r) / 2; 
            // println!("{}", mid);
            if nums[mid] > target {
                r = mid - 1;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                return mid as i32;
            }
        }
        -1
    }
}

// @lc code=end


mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let n = vec![-1,0,3,5,9,12];
        let t = 2;
        Solution::search(n, t);
    }
}

