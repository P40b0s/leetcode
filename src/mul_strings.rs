struct MultString
{
    val: String
}

impl MultString
{
    fn new(val: &str) -> Self
    {
        if val.len() > 1
        {
            MultString { val: val.trim_start_matches("0").to_owned() }
        }
        else
        {
            MultString { val: val.to_owned() }
        }
    }
    fn get_value(&self) -> String
    {
        self.val.clone()
    }
}

impl Add for MultString
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output 
    {
        let mut result: String = String::new();
        let mut mem: bool = false;
        let mut first: String = String::new();
        let mut second: String = String::new();
        if self.val.len() > rhs.val.len()
        {
            first = self.val.chars().rev().collect::<String>();
            second = rhs.val.chars().rev().collect::<String>();
        }
        else
        {
            first = rhs.val.chars().rev().collect::<String>();
            second = self.val.chars().rev().collect::<String>();
        }
        for (i, v1) in first.chars().enumerate()
        {
            let f = v1.to_digit(10).unwrap();
            let mut r = f;
            if let Some(s) = second.chars().nth(i)
            {
                let s = s.to_digit(10).unwrap();
                r = f + s;
            }
            let sum = match mem
            {
                true => 
                {
                    mem = false;
                    r + 1
                },
                false => r
            };
            if sum > 9
            {
                mem = true;
                r = sum%10;
            }
            else
            {
                mem = false;
                r = sum;
            }
            let ch = char::from_digit(r, 10);
            result.push(ch.unwrap());
        }
        if mem
        {
            let ch = char::from_digit(1, 10);
            result.push(ch.unwrap());
        }
        MultString { val: result.chars().rev().collect::<String>() }
    }
}

impl Mul for MultString
{
   type Output = Self;
   fn mul(self, rhs: Self) -> Self::Output 
   {
        let mut partial: Vec<String> = vec![];
        let mut mem: Option<u32> = None;
        let mut zeros_len = 0;
        let mut first: String = String::new();
        let mut second: String = String::new();
        if self.val.starts_with("0") || rhs.val.starts_with("0")
        {
            return  MultString{val: "0".to_owned()};
        }
        if self.val.len() > rhs.val.len()
        {
            first = self.val.chars().rev().collect::<String>();
            second = rhs.val.chars().rev().collect::<String>();
        }
        else
        {
            first = rhs.val.chars().rev().collect::<String>();
            second = self.val.chars().rev().collect::<String>();
        }
        let zeros_f = first.chars().take_while(|t| t.to_digit(10).unwrap() == 0).count();
        let zeros_s = second.chars().take_while(|t| t.to_digit(10).unwrap() == 0).count();
        zeros_len = zeros_f + zeros_s;
        let first = &first[zeros_f..];
        let second = &second[zeros_s..];
        for s in second.chars()
        {
            let mut part_item = String::new();
            for (i, v1) in first.chars().enumerate()
            {
                let f = v1.to_digit(10).unwrap();
                let s = s.to_digit(10).unwrap();
                let mut r = f * s;
                let sum = match mem
                {
                    Some(m) => r + m,
                    None => r
                };
                if sum > 9
                {
                    mem = Some((sum/10)%10);
                    r = sum%10;
                    if first.len() - 1 == i
                    {
                        r = sum;
                        mem = None;
                    }
                }
                else
                {
                    mem = None;
                    r = sum;
                }
                if r > 9
                {
                    let n1 = (r/10)%10;
                    let n2 = r%10;
                    let ch1 = char::from_digit(n1, 10);
                    let ch2 = char::from_digit(n2, 10);
                    part_item.push(ch2.unwrap());
                    part_item.push(ch1.unwrap());
                    mem = None;
                }
                else
                {
                    let ch = char::from_digit(r, 10);
                    part_item.push(ch.unwrap());
                }
            }
            let part_item = part_item.chars().rev().collect::<String>();
            partial.push(part_item);
           
        }
        let mut ms: Option<MultString> = None;
        for (z, st) in partial.into_iter().enumerate()
        {
            if let Some(val) = ms
            {
                ms = Some(val + MultString{val: [st, "0".repeat(z)].concat()});
            }
            else
            {
                let pref = mem.and_then(|a| char::from_digit(a, 10).as_ref().and_then(|t| Some(t.to_string()))).unwrap_or(String::new());
                ms = Some(MultString{val: [pref, st].concat()});
            }
        }
        if zeros_len > 0
        {
            let z = ms.unwrap();
            let zz = "0".repeat(zeros_len);
            let zeros = [z.val, zz].concat();
            MultString{val: zeros}
        }
        else
        {
            ms.unwrap()
        }
        
   }
} 

// fn multiply(a: &str, b: &str) -> String 
// {
    
//     let mut one = a;
//     if a.len() > 1
//     {
//         one = a.trim_start_matches("0");
//     }
//     let mut two = b.trim_start_matches("0");
//     if b.len() > 1
//     {
//         two = b.trim_start_matches("0");
//     }
//     18446744073709551615


// }

// fn split(s: &str)
// {
//     let mut arr : Vec<String> = vec![];
//     if s.contains("0")
//     {
//         let zeros = s.split("0").collect::<Vec<&str>>();
//         for z in zeros
//         {
//             if zeros.len() > 20
//             {
                
//             }
//         }
//     }
//     else
//     {

//     }
// }

use std::{ops::{Add, Mul}, time::Instant};



#[cfg(test)]
mod tests
{

    use super::MultString;

   
    #[test]
    fn test_plus()
    {
        let m1 = MultString::new("1234567890");
        let m2 = MultString::new("9876543210");
        let res = m1 + m2;
        println!("{}", res.val );

        
      
    }
    #[test]
    fn test_mult()
    {
        // let m1 = MultString{val : "365400".to_owned()};
        // let m2 = MultString{val : "3".to_owned()};
        // let res = m1 * m2;
        // assert_eq!(res.val, "1096200");

        // let m1 = MultString{val : "3912".to_owned()};
        // let m2 = MultString{val : "6".to_owned()};
        // let res = m1 * m2;
        // assert_eq!(res.val, "23472");

        // let m1 = MultString{val : "4235".to_owned()};
        // let m2 = MultString{val : "86".to_owned()};
        // let res = m1 * m2;
        // assert_eq!(res.val, "364210");

        // let m1 = MultString{val : "1854".to_owned()};
        // let m2 = MultString{val : "237".to_owned()};
        // let res = m1 * m2;
        // assert_eq!(res.val, "439398");

        let m1 = MultString{val : "58608473622772837728372827".to_owned()};
        let m2 = MultString{val : "7586374672263726736374".to_owned()};
        let res = m1 * m2;
        assert_eq!(res.get_value(), "444625839871840560024489175424316205566214109298");

        let m1 = MultString::new("2");
        let m2 = MultString::new("0");
        let res = m1 * m2;
        assert_eq!(res.val, "0");

        let m1 = MultString::new("0000001");
        let m2 = MultString::new("3");
        let res = m1 * m2;
        assert_eq!(res.val, "3");

        let m1 = MultString::new("1009");
        let m2 = MultString::new("03");
        let res = m1 * m2;
        assert_eq!(res.val, "3027");


        // do_test("98765", "56894", "5619135910");
        // do_test("9007199254740991", "9007199254740991", "81129638414606663681390495662081");
        // do_test("1020303004875647366210", "2774537626200857473632627613", 
        //         "2830869077153280552556547081187254342445169156730");
        // do_test("58608473622772837728372827", "7586374672263726736374",
        //         "444625839871840560024489175424316205566214109298");

        //println!("{}", res.val );

        
      
    }

    #[test]
    fn test_ost()
    {
        println!("{}", (65/10)%10);

        
      
    }
}