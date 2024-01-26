
//[1,2,3,4,5,6,7,8]

use std::ops::{Add, DerefMut};

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 
{
    let mut merged = nums1;
    merged.extend(nums2);
    merged.sort();
    let len = merged.len();
    let mid = len / 2;
    if len % 2 == 0
    {
        let  sum = merged[mid-1..=mid].iter_mut().fold(0, |acc, v| 
        {
            *v += acc;
            *v
        });
        return sum as f64/2.0f64;
    }
    else
    {
        return merged[mid] as f64;
    }
}

#[test]
fn test_mid()
{
    let r = find_median_sorted_arrays(vec![1,2,3,4,5,6], vec![8,9,10]);
    println!("{}", r);
}