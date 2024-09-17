use std::collections::HashMap;

//https://leetcode.com/problems/longest-substring-without-repeating-characters/
pub struct Solution;
impl Solution 
{
    pub fn longest_palindrome(s: String) -> String 
    {
        let mut s2: Vec<char> = vec!['#'; s.len() * 2 + 1];
        for s1 in s.char_indices() 
        {
            s2[s1.0 * 2 + 1] = s1.1;
        }
        let mut p = vec![0; s2.len()];
        let mut r = 0;
        let mut c = 0;
        let mut index = 0;
        let mut max_len = 1;
        for (i, _) in s2.iter().enumerate()
        {
            let i_mir = if let Some(cs) = ((2*c)as i32).checked_sub(i as i32)
            {
                cs as usize
            }
            else
            {
                0 as usize
            };
            if r > i
            {
                p[i] = std::cmp::min(p[i_mir], r - i);
            }
            else 
            {
                p[i] = 0;
            }
            while i > p[i]
            && (i + p[i] + 1) < s2.len()
            && s2[i - p[i] - 1] == s2[i + p[i] + 1]
            {
                p[i] += 1;
            }
            if p[i] + i > r
            {
                c = i;
                r = i + p[i];
            }
            if max_len < p[i]
            {
                max_len = p[i];
                index = i;
            }
        }
        s[(index - max_len)/2..max_len].to_string()
    }

    fn longest_palindrome_2(s: String) -> String 
    {
        let l = s.len();
        if l <= 1 
        {
            return s;
        }
        let mut chars: Vec<char> = vec!['#'; s.len() * 2 + 1];
        for s1 in s.char_indices() 
        {
            chars[s1.0 * 2 + 1] = s1.1;
        }
        let mut length_of_palindrome = vec![1usize; chars.len()];
        let mut current_center: usize = 0;
        let mut right_from_current_center: usize = 0;

        for i in 0..chars.len() 
        {
            if right_from_current_center > i && i > current_center 
            {
                length_of_palindrome[i] = std::cmp::min(right_from_current_center - i, length_of_palindrome[2 * current_center - i]);
                if length_of_palindrome[i] + i >= right_from_current_center 
                {
                    current_center = i;
                    right_from_current_center = length_of_palindrome[i] + i;
                    if right_from_current_center >= chars.len() - 1 
                    {
                        break;
                    }
                } 
                else 
                {
                    continue;
                }
            }

            let mut radius: usize = (length_of_palindrome[i] - 1) / 2;
            radius += 1;
            while i >= radius && i + radius <= chars.len() - 1 && chars[i - radius] == chars[i + radius]
            {
                length_of_palindrome[i] += 2;
                radius += 1;
            }
        }
        let center_of_max = length_of_palindrome
            .iter()
            .enumerate()
            .max_by_key(|(_, &value)| value)
            .map(|(idx, _)| idx)
            .unwrap();
        let radius_of_max = (length_of_palindrome[center_of_max] - 1) / 2;
        let answer = &chars[(center_of_max - radius_of_max)..(center_of_max + radius_of_max + 1)]
            .iter()
            .collect::<String>();
        answer.replace('#', "")
        }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        let r = super::Solution::longest_palindrome("babad".to_string());
        println!("Длинна {}", r);
    }
    #[test]
    fn test_2()
    {
        let r = super::Solution::longest_palindrome("cbbd".to_string());
        println!("Длинна {}", r);
    }

    #[test]
    fn test_1_2()
    {
        let r = super::Solution::longest_palindrome_2("babad".to_string());
        println!("Длинна {}", r);
    }
    #[test]
    fn test_2_2()
    {
        let r = super::Solution::longest_palindrome_2("cbbd".to_string());
        println!("Длинна {}", r);
    }
}
