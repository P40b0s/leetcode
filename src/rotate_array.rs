pub struct Solution{}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) 
    {
        if k as usize > nums.len()
        {
            for _ in 0..k
            {
                let left_range = 0..nums.len() - 1 as usize;
                let right_range = nums.len() - 1 as usize..nums.len();
                let cnc = [&nums[right_range], &nums[left_range]].concat();
                *nums = cnc;
            }
        }
        else 
        {
            let left_range = 0..nums.len() - k as usize;
            let right_range = nums.len() - k as usize..nums.len();
            let cnc = [&nums[right_range], &nums[left_range]].concat();
            *nums = cnc;
        }
    }
}
#[cfg(test)]
mod tests
{
    #[test]
    fn tsts()
    {
        let mut v = vec![1,2,3,4,5,6,7];        
        super::Solution::rotate(&mut v, 3);
        assert_eq!(vec![5,6,7,1,2,3,4], v);
    }
    #[test]
    fn tsts2()
    {
        let mut v = vec![-1,-100,3,99];        
        super::Solution::rotate(&mut v, 2);
        assert_eq!(vec![3,99,-1,-100], v);
    }
}