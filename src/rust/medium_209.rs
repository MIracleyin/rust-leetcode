/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] 长度最小的子数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_sub_array_len_v1(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum >= target {
                    let sublen = (j - i + 1) as i32;
                    if sublen < res{
                        res = sublen;
                    }
                    break;
                }
            }
        }
        if res == i32::MAX {
            return 0;
        } else {
            res
        }
    }

    pub fn min_sub_array_len_v2(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut sum = 0;
        let mut lw = 0;
        let mut sub_len = 0;

        for pos in 0..nums.len() {
            sum += nums[pos];
            while (sum >= target) {
                sub_len = (pos - lw + 1) as i32;
                if sub_len < res {
                    res = sub_len;
                }
                sum -= nums[lw];
                lw += 1;
            }
        }

        if res == i32::MAX {
            return 0;
        } else {
            res
        }
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut res, mut sublen) = (i32::MAX, 0);
        let (mut sum, mut l) = (0, 0);

        for (pos, val) in nums.iter().enumerate() {
            sum += val;
            while (sum >= target) {
                sublen = (pos - l + 1) as i32;
                if sublen < res {
                    res = sublen;
                }
                sum -= nums[l];
                l += 1;
            }
        }
        if res == i32::MAX {
            res = 0;
        }
        
        res
    
    }

}
// @lc code=end

mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let n = vec![2,3,1,2,4,3];
        let t = 7;
        let size = Solution::min_sub_array_len(t, n);
        // println!("{:?}", n);
        println!("{}", size);
    }
}