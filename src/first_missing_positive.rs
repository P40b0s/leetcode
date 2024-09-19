use std::collections::HashSet;



fn solution(nums: Vec<i32>) -> i32 
{
    let mut nums = nums;
    let hs: HashSet<_> = nums.drain(..).collect();
    nums.extend(hs.into_iter());
    nums.sort();
    if let Some(first) = nums.iter().position(|f|f.is_positive())
    {
        for (i, n) in nums[first..].iter().enumerate()
        {
            if *n != (i+1) as i32
            {
                return (i+1) as i32;
            }
        }
    }
    let last = nums[nums.len() - 1] + 1;
    if last == 0 || last.is_negative()
    {
        return 1;
    }
    else 
    {
        return last
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        println!("{}", super::solution(vec![0,11,10,22,13,2,1,3,4,5,6,21]));
    }
    #[test]
    fn test_2()
    {
        println!("{}", super::solution(vec![0,2,1]));
    }
    #[test]
    fn test_3()
    {
        println!("{}", super::solution(vec![0,2,2,1,1]));
    }
    #[test]
    fn test_4()
    {
        println!("{}", super::solution(vec![-5]));
    }
    #[test]
    fn test_5()
    {
        println!("{}", super::solution(vec![-1, -2]));
    }
}