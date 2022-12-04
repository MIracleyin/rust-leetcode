struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let len = s.len();
        let n = n as usize;
        let mut s = s.chars().collect::<Vec<char>>();

        Self::reverse(&mut s, 0, n);
        Self::reverse(&mut s, n + 1, len - 1);
        Self::reverse(&mut s, 0, len - 1);

        s.iter().collect::<String>()
    }

    pub fn reverse(s: &mut Vec<char>, mut l: usize, mut r: usize) {
        while l < r {
            let tc = s[l];
            s[l] = s[r];
            s[r] = tc;
            l += 1;
            r -= 1;
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "We are happy.".to_string();
        let res = Solution::reverse_left_words(s, 5);
        dbg!(res);
    }
}
