use std::collections::HashMap;

//https://leetcode.com/problems/longest-substring-without-repeating-characters/
pub struct Solution;
impl Solution 
{
    pub fn length_of_longest_substring(s: String) -> i32 
    {
        let mut chars = Vec::new();
        let mut  n = 0;
        for ch in s.chars()
        {
            let splitted = chars.iter().position(|c| c == &ch);
            if let Some(splitted) = splitted
            {
                if chars.len() > n
                {
                    n = chars.len();
                }
                let split = chars.split_at(splitted + 1);
                chars = split.1.to_vec();
                chars.push(ch);
            }
            else 
            {
                chars.push(ch);
                if chars.len() > n
                {
                    n = chars.len();
                }
            }
        }
        n as i32
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_len_1()
    {
        let r = super::Solution::length_of_longest_substring("abcabcbb".to_string());
        println!("Длинна {}", r);
    }
    #[test]
    fn test_len_2()
    {
        let r = super::Solution::length_of_longest_substring("dvdf".to_string());
        println!("Длинна {}", r);
    }
    #[test]
    fn test_len_3()
    {
        let r = super::Solution::length_of_longest_substring("pwwkew".to_string());
        println!("Длинна {}", r);
    }
}
