

/*
 * @lc app=leetcode id=139 lang=rust
 *
 * [139] Word Break
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn word_break_v1(s: String, word_dict: Vec<String>) -> bool {
        fn helper<'a>(
            cache: &mut HashMap<&'a str, bool>,
            s: &'a str,
            wd: &Vec<String>,
        ) -> bool {
            match cache.get(s) {
                Some(ans) => *ans,
                None => {
                    for word in wd {
                        if s.starts_with(word) && (s.len() == word.len() || helper(cache, &s[word.len()..], wd)){
                            cache.insert(s, true);
                            return true;
                        }
                    }
                    cache.insert(s, false);
                    false
                }
            }
        }
        
        helper(&mut HashMap::new(), &s, &word_dict)
    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // dp[i] : 字符串长度为i的话，dp[i]为true，表示可以拆分为一个或多个在字典中出现的单词。
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..s.len() {
            if dp[i] == false {
                continue;
            }
            for word in word_dict.iter() {
                if let Some(substr) = s.get(i..i+word.len()) {
                    if substr == *word {
                        dp[i + word.len()] = true;
                    }
                }
            }
        }
        dp[s.len()]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet", "code"];
    let word_dict = word_dict.into_iter().map(String::from).collect();
    let res = Solution::word_break(s, word_dict);
    dbg!(res);
}