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
        let mut chars: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
        for c in s.chars() {
            chars.push('#');
            chars.push(c);
        }
        chars.push('#');
        // let mut chars: Vec<char> = vec!['#'; s.len() * 2 + 1];
        // for s1 in s.char_indices() 
        // {
        //     chars[s1.0 * 2 + 1] = s1.1;
        // }

        // List: storing the length of palindrome at each index of string
        let mut length_of_palindrome = vec![1usize; chars.len()];
        // Integer: Current checking palindrome's center index
        let mut current_center: usize = 0;
        // Integer: Right edge index existing the radius away from current center
        let mut right_from_current_center: usize = 0;

        for i in 0..chars.len() {
            // 1: Check if we are looking at right side of palindrome.
            if right_from_current_center > i && i > current_center {
                // 1-1: If so copy from the left side of palindrome.
                // If the value + index exceeds the right edge index, we should cut and check palindrome later #3.
                length_of_palindrome[i] = std::cmp::min(
                    right_from_current_center - i,
                    length_of_palindrome[2 * current_center - i],
                );
                // 1-2: Move the checking palindrome to new index if it exceeds the right edge.
                if length_of_palindrome[i] + i >= right_from_current_center {
                    current_center = i;
                    right_from_current_center = length_of_palindrome[i] + i;
                    // 1-3: If radius exceeds the end of list, it means checking is over.
                    // You will never get the larger value because the string will get only shorter.
                    if right_from_current_center >= chars.len() - 1 {
                        break;
                    }
                } else {
                    // 1-4: If the checking index doesn't exceeds the right edge,
                    // it means the length is just as same as the left side.
                    // You don't need to check anymore.
                    continue;
                }
            }

            // Integer: Current radius from checking index
            // If it's copied from left side and more than 1,
            // it means it's ensured so you don't need to check inside radius.
            let mut radius: usize = (length_of_palindrome[i] - 1) / 2;
            radius += 1;
            // 2: Checking palindrome.
            // Need to care about overflow usize.
            while i >= radius && i + radius <= chars.len() - 1 && chars[i - radius] == chars[i + radius]
            {
                length_of_palindrome[i] += 2;
                radius += 1;
            }
        }

        // 3: Find the maximum length and generate answer.
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
    fn test_1_1()
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
