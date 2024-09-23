use std::collections::{HashMap, HashSet};


pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 
{
    let mut len = s.chars().count();
    let mut s = s;
    let mut dictionary = dictionary;
    dictionary.sort_by_key(|s| s.len());
    dictionary.reverse();
    for w in &dictionary
    {
        while let Some(i) = s.find(w)
        {
            let w_len = w.chars().count();
            len -= w_len;
            s.replace_range(i..(i + w_len), "");
            println!("-{} = {}", w, s);
        }
    }
    len as i32
}


struct Solution;

impl Solution 
{
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 
    {
        let dictionary_set: HashSet<&str> = dictionary.iter().map(|word| word.as_str()).collect();
        let mut dp = vec![i32::MAX; s.len()];
        Self::split(&s, 0, &mut dp, &dictionary_set)
    }
    
    fn split(s: &String, index: usize, dp: &mut Vec<i32>, dictionary: &HashSet<&str>) -> i32 
    {
        if index >= s.len() 
        {
            return 0;
        }
        if dictionary.contains(&s[index..]) 
        {
            return 0;
        }
        if dp[index] != i32::MAX 
        {
            return dp[index];
        }
        let mut min = (s.len() - index) as i32;
        for i in (index + 1)..=s.len() 
        {
            let count = if dictionary.contains(&s[index..i]) 
            { 
                0 
            }
            else 
            { 
                (i - index) as i32 
            };
            let count = count + Self::split(s, i, dp, dictionary);
            min = min.min(count);
        }
        dp[index] = min;
        min
    }
}

struct Solution2;
impl Solution2 
{ 
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32
    {
        let dict: HashSet<&str> = dictionary.iter().map(|word| word.as_str()).collect();
        let n = s.len();
        let mut dp = vec![n as i32 + 1; n + 1];
        dp[0] = 0; 
        for i in 1..=n 
        {
            dp[i] = dp[i - 1] + 1;
            for j in 0..i 
            {
                if dict.contains(&s[j..i])
                {
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }
        dp[n]
    }
}
struct Solution3;
impl Solution3 
{ 
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32
    {
        let dict: HashSet<&str> = dictionary.iter().map(|word| word.as_str()).collect();
        let n = s.len();
        let mut dp = vec![n as i32 + 1; n + 1];
        dp[0] = 0; 
        for i in 1..=n 
        {
            dp[i] = dp[i - 1] + 1;
            for j in 0..i 
            {
                if dict.contains(&s[j..i])
                {
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests
{
    use crate::merge_ksorted_list::ListNode;
    #[test]
    fn test_1()
    {
        let points = vec!["leet".to_string(),"code".to_string(),"leetcode".to_string()];
        let s  = "leetscode";
        println!("{:?}", super::min_extra_char(s.to_string(), points));
    }
    #[test]
    fn test_2()
    {
        let points = vec!["hello".to_string(),"world".to_string()];
        let s  = "sayhelloworld";
        println!("{:?}", super::min_extra_char(s.to_string(), points));
    }
    #[test]
    fn test_3()
    {
        let points = vec!["na","i","edd","wobow","kecv","b","n","or","jj","zul","vk","yeb","qnfac","azv","grtjba","yswmjn","xowio","u","xi","pcmatm","maqnv"].into_iter().map(|m| m.to_string()).collect();
        let s  = "azvzulhlwxwobowijiyebeaskecvtjqwkmaqnvnaomaqnvf";
        println!("{:?}", super::min_extra_char(s.to_string(), points));  

    }
    #[test]
    fn test_4()
    {
        let points = vec!["na","i","edd","wobow","kecv","b","n","or","jj","zul","vk","yeb","qnfac","azv","grtjba","yswmjn","xowio","u","xi","pcmatm","maqnv"].into_iter().map(|m| m.to_string()).collect();
        let s  = "azvzulhlwxwobowijiyebeaskecvtjqwkmaqnvnaomaqnvf";
        println!("{:?}", super::Solution2::min_extra_char(s.to_string(), points));  

    }
    // azvzulhlwxwobowijiyebeaskecvtjqwkmaqnvnaomaqnvf
    // azvzulhlwxwobowijiyebeaskecvtjqwknaomaqnvf
    // azvzulhlwxwobowijiyebeaskecvtjqwknaof
    // azvzulhlwxijiyebeaskecvtjqwknaof
    // azvzulhlwxijiyebeastjqwknaof
    // zulhlwxijiyebeastjqwknaof
    // zulhlwxijieastjqwknaof
    // hlwxijieastjqwknaof
    // hlwjieastjqwknaof
    // hlwjieastjqwkof
    // hlwjeastjqwkof
    // hlwjeastjqwkof


}