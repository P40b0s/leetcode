

fn solution(height: Vec<i32>) -> i32 
{
    let mut count = 0i32;
    let mut skip = 0;
    for (i, t) in height.iter().enumerate()
    {
        if i < skip
        {
            continue;
        }
        if t == &0
        {
            continue;
        }
        let next = i+1;
        if next < height.len()
        {
            if &height[next] > t
            {
                continue;
            }
        }
        let mut t = *t;
        let mut trap_end = height[i+1..].iter().position(|f| f >= &t);
        while trap_end.is_none()
        {
            if t == 0
            {
                break;
            }
            t -= 1;
            trap_end = height[i+1..].iter().position(|f| f >= &t);
        }
        if let Some(trap_end) = trap_end
        {
            let trap_range = &height[i+1..(i + 1 + trap_end)];
            for tr in trap_range
            {
                count += &t - tr;
            }
            skip = i + 1 + trap_end;
        }
    }
    count
}
#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        println!("{}", super::solution(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
    }
    #[test]
    fn test_2()
    {
        println!("{}", super::solution(vec![4,2,0,3,2,5]));
    }
    #[test]
    fn test_2_2()
    {
        println!("{}", super::solution(vec![5,2,3,0,4,2]));
    }
    #[test]
    fn test_3()
    {
        println!("{}", super::solution(vec![4,2,3]));
    }
}