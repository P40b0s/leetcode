// if D = 0 then error(DivisionByZeroException) end -- Деление на ноль
// Q := 0                  -- Начальные значения частного и остатка полагаем равны 0
// R := 0                     
// for i := n − 1 .. 0 do  -- Здесь n равно числу бит в N
//   R := R << 1           -- Сдвиг влево числа R на 1 бит
//   R(0) := N(i)          -- Полагаем младший бит R равным биту i делимого
//   if R >= D then
//     R := R − D
//     Q(i) := 1
//   end
// end
pub fn divide(dividend: i32, divisor: i32) -> i32 
{
    if divisor == 0
    {
        return 0;
    }
    let negative_1 = dividend.is_negative();
    let negative_2 = divisor.is_negative();
    let d1 = dividend.to_string();
    let d2 = divisor.to_string();
    let dividend = if negative_1
    {
        d1[1..d1.len()].parse::<u32>().unwrap()
    }
    else
    {
        dividend as u32
    };
    let divisor = if negative_2
    {
        d2[1..d2.len()].parse::<u32>().unwrap()
    }
    else
    {
        divisor as u32
    };
    let mut d = 0u32;
    let mut yd = 0u32;
    while yd <= dividend
    {
        d += 1;
        yd += divisor;
    }
    if negative_1 && negative_2
    {
        if (d-1) > (i32::MAX as u32)
        {
            return i32::MAX;
        }
        else
        {
            return (d-1) as i32;
        }
    }
    if negative_1 || negative_2
    {
        if (d-1) > (i32::MIN as u32)
        {
            return i32::MIN
        }
        else 
        {
            return ["-".to_owned(), (d-1).to_string()].concat().parse::<i32>().unwrap();
        }
        // if let Ok(parsed) = [(d-1).to_string()].concat().parse::<i32>()
        // {
        //     return ["-".to_owned(), parsed.to_string()].concat().parse::<i32>().unwrap();
        // }
        // else 
        // {
        //     return i32::MIN;  
        // }
    }
    (d-1) as i32
    
    
}

#[test]
fn test_fn()
{
    let res = divide(-2147483648, 1);
    println!("{}", res);
}
