

fn divide(dividend: i32, divisor: i32) -> i32
{
    if dividend == 0
    {
        return 0;
    }
    let sign: bool = (dividend < 0) ^ (divisor < 0);
    let dividend =  (dividend as i64).abs();
    let divisor = (divisor as i64).abs();
    let mut quo = 0i64;
    let mut temp = 0i64;
    for i in (0..=31).rev()
    {
        if temp + ((divisor) << i) <= dividend
        {
            temp += divisor << i;
            quo |= 1 << i;
        }
    }
    if sign == false
    {
        if quo > i32::MAX.into()
        {
            return i32::MAX;
        }
        else
        {
            return quo as i32;
        }
    }
    else 
    {
        return -quo as i32;
    }
}
#[cfg(test)]
mod tests
{
    #[test]
    fn test_1()
    {
        println!("{}", super::divide(-2147483648i32, -1));
    }
    #[test]
    fn test_2()
    {
       
        println!("{}", super::divide(10, -3));
    }
    #[test]
    fn test_3()
    {
       
        println!("{}", super::divide(-1, -1));
    }
    #[test]
    fn test_4()
    {
       
        println!("{}", super::divide(-2147483648, 2));
    }
}