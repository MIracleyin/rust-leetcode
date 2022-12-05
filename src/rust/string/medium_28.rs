/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn get_next_v1(s: &Vec<char>) -> Vec<usize> {
        let len = s.len();
        let mut j = 0;
        let mut next = vec![0; len];
        for i in 1..len {
            // i0 = j + 1
            while j > 0 && s[i] != s[j] {
                //
                j = next[j - 1]; // perfix table
            }

            if s[i] == s[j] {
                j += 1 
            }

            next[i] = j; // j is per post same string len
        }
        next
    }

    pub fn str_str_v1(haystack: String, needle: String) -> i32 {
        let (h_len, n_len) = (haystack.len(), needle.len());
        if h_len == 0 {
            return 0;
        }
        if h_len < n_len {
            return -1;
        }
        let (haystack, needle) = (
            haystack.chars().collect::<Vec<char>>(),
            needle.chars().collect::<Vec<char>>(),
        );

        let next = Self::get_next_v1(&needle);

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

    pub fn get_next(s: &Vec<char>) -> Vec<i32>{
        let len = s.len();
        let mut next = vec![-1; len];
        let mut j = -1;
        for i in 1..len { // here i from 1 because next[i] always is 0
            // s[i] != s[j]
            while j > -1 && s[i] != s[(j + 1) as usize] {
                j = next[j as usize] // iter prefix table
            }
            // s[i] s[j]
            if s[i] == s[(j + 1) as usize] {
                j += 1;
            }
            next[i] = j;
        }

        next
    }


    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (h_len, n_len) = (haystack.len(), needle.len());
        if h_len == 0 {
            return 0;
        }
        if h_len < n_len {
            return -1;
        }

        let (haystack, needle) = (
            haystack.chars().collect::<Vec<char>>(),
            needle.chars().collect::<Vec<char>>(),
        );



        let mut j = -1;
        let next = Self::get_next(&needle);
        for i in 0..h_len {
            while j > -1 && haystack[i] != needle[(j + 1) as usize] {
                j = next[j as usize]
            }
            if haystack[i] == needle[(j + 1) as usize] {
                j += 1;
            }
            if j == n_len as i32 - 1{ // full match
                return (i - n_len + 1) as i32
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
        // let res = Solution::str_str(h, n);
        let n = n.chars().collect::<Vec<char>>();
        let res = Solution::get_next(&n); // 010120
        dbg!(res);
    }
}
