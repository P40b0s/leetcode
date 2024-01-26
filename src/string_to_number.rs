///https://leetcode.com/problems/string-to-integer-atoi/submissions/
pub fn my_atoi(s: String) -> i32
{
    let trimmed = s.trim_start();
    if trimmed.len() == 0
    {
        return 0;
    }
    let mut for_parse = "".to_owned();
    for (i, ch) in trimmed.chars().enumerate()
    {
        if i == 0 && ch == '-'
        {
            for_parse.push('-');
            continue;
        }
        else if i == 0 && ch == '+'
        {
            continue;
        } 
        else if !ch.is_digit(10)
        {
            break;
        }
        for_parse.push(ch);
    }
    let res = for_parse.parse::<i32>();
    if res.is_err() 
    {
       let result = match res.err().unwrap().kind() 
       {
           std::num::IntErrorKind::PosOverflow => i32::MAX,
           std::num::IntErrorKind::NegOverflow => i32::MIN,
           _ => 0
       };
       return result;
    }
    else
    {
       return res.unwrap();
    }
}


#[test]
fn test_fn()
{
    let sl = String::from("  -423");
    let res = my_atoi(sl);
    println!("{}", res);
}