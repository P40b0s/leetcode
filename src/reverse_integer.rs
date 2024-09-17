use std::collections::HashMap;

//https://leetcode.com/problems/longest-substring-without-repeating-characters/
pub struct Solution;
impl Solution 
{
    pub fn solution(x: i32) -> i32
    {
        let rad = 10;
        let mut s = x;
        let mut reversed: i32 = 0;
        while s != 0 
        {
            let mul = reversed.checked_mul(rad);
            if let Some(mul) = mul
            {
                reversed = mul + (s % rad);
                s /= rad;
            }
            else 
            {
                reversed = 0;
                break;    
            }
        }
        reversed
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        let r = super::Solution::solution(-321);
        println!("{}", r);
    }
    #[test]
    fn test_2()
    {
        let r = super::Solution::solution(120);
        println!("{}", r);
    }
    #[test]
    fn test_3()
    {
        let r = super::Solution::solution(1534236469);
        println!("{}", r);
    }
    

}
