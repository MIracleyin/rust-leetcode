/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn get_next(next: &mut Vec<usize>, s: &Vec<char>) {
        let len = s.len();
        let mut j = 0;
        for i in 1..len { // i0 = j + 1
            while j > 0 && s[i] != s[j] { // 
                j = next[j - 1];
            }

            if s[i] == s[j] {
                j += 1
            }

            next[i] = j; // j is per post same string len
        }
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (h_len, n_len) = (haystack.len(), needle.len());
        if h_len == 0 {
            return 0;
        }
        if h_len < n_len {
            return -1;
        }
        let (haystack, needle) = (haystack.chars().collect::<Vec<char>>(), needle.chars().collect::<Vec<char>>());

        let mut next = vec![0; n_len];
        Self::get_next(&mut next, &needle);

        let mut j = 0;
        for i in 0..h_len {
            while j > 0 && haystack[i] != needle[j] {
                 j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == n_len {
                return (i - n_len + 1) as i32;
            }
        }
        -1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let h = "aabaabaaf".to_string();
        let n = "aabaaf".to_string();
        let res = Solution::str_str(h, n);
        dbg!(res);
    }
}

