use std::collections::{HashMap, HashSet};


pub fn is_match(s: String, p: String) -> bool
{
    let str = &s;
    let pattern = &p;
    let m = str.len();
    let n = pattern.len();
    let mut prev = vec![false; n + 1];
    let mut curr = vec![false; n + 1];
    prev[0] = true;
    for j in 1..=n 
    {
        let mut flag = true;
        for k in 1..=j 
        {
            if pattern.chars().nth(k - 1) != Some('*') 
            {
                flag = false;
                break;
            }
        }
        prev[j] = flag;
    }
    for i in 1..=m 
    {
        for j in 1..=n 
        {
            if str.chars().nth(i - 1) == pattern.chars().nth(j - 1) || pattern.chars().nth(j - 1) == Some('?') 
            {
                curr[j] = prev[j - 1];
            } 
            else if pattern.chars().nth(j - 1) == Some('*') 
            {
                curr[j] = prev[j] || curr[j - 1];
            } 
            else 
            {
                curr[j] = false;
            }
        }
        prev.copy_from_slice(&curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        let s = "cb".to_string();
        let p = "?b".to_string();
        println!("{}", super::is_match(s, p));
    }
}