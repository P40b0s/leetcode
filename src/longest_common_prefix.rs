use std::collections::{HashMap, HashSet};


pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 
{
    
    let mut count: i32 = 0;
    let mut pref_1 : HashSet<i32> =  HashSet::new();
    for mut a1 in arr1
    {
        while a1 > 0
        {
            pref_1.insert(a1);
            a1 /= 10;
        }
    }
    for mut a2 in arr2
    {
        while !pref_1.contains(&a2) && a2 > 0
        {
            a2 /= 10;
        }
        if a2 > 0
        {
            count = count.max((a2.ilog10() +1) as i32);
        }
    }

    // for a in arr2
    // {
    //     let mut r = a;
    //     if arr1.contains(&a)
    //     {
    //         let dc = dig_count(&r);
    //         if count < dc
    //         {
    //             count = dc;
    //         }
    //     }
    //     else 
    //     {
    //         while r > 0
    //         {
    //             r = r / 10;
    //             if arr1.contains(&r)
    //             {
    //                 let dc = dig_count(&r);
    //                 if count < dc
    //                 {
    //                     count = dc;
    //                 }
    //             }
    //         }
    //         if arr1.contains(&r)
    //         {
    //             if count == 0
    //             {
    //                 count = 1;
    //             }
    //         }
    //     }
    // }
    count
}


fn dig_count(d: &i32) -> i32
{
    let mut d = *d;
    let mut c = 0;
    while d > 0
    {
        d /= 10;
        c += 1;
    }
    c
}


#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        let arr1 = vec![1, 10, 100];
        let arr2  = vec![1000];
        println!("{:?}", super::longest_common_prefix(arr1, arr2));
    }
}