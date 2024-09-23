use std::collections::{HashMap, HashSet};

fn gcd(a: i32, b: i32) -> i32 
{
    if b == 0 
    {
        a.abs()
    } 
    else 
    {
        gcd(b, a % b)
    }
}

fn max_points(points: Vec<Vec<i32>>) -> i32 
{
    if points.len() < 3 
    {
        return points.len() as i32;
    }
    let mut max_count = 0;
    for i in 0..points.len() 
    {
        let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
        let mut same_point_count = 1; 
        for j in (i + 1)..points.len() 
        {
            let dx = points[j][0] - points[i][0];
            let dy = points[j][1] - points[i][1];
            if dx == 0 && dy == 0 
            {
                same_point_count += 1;
                continue;
            }
            // Вычисляем НОД для нормализации угла наклона
            let g = gcd(dx, dy);
            let normalized_slope = (dy / g, dx / g); // Обеспечиваем согласованное представление
            // Normalize the slope to have a consistent direction
            let slope = if normalized_slope.1 < 0 
            { 
                (-normalized_slope.0, -normalized_slope.1) 
            } 
            else if normalized_slope.1 == 0 
            {
                (1, 0)
            } 
            else 
            {
                normalized_slope
            };
            *slopes.entry(slope).or_insert(0) += 1; // Увеличиваем счетчик угла наклона
        }
        let mut current_max = same_point_count; // Начинаем с одинаковых точек
        for &count in slopes.values() 
        {
            current_max = current_max.max(count + same_point_count);
        }
        max_count = max_count.max(current_max);
    }
    max_count
}






// fn calc_dia(points: &mut Vec<Vec<i32>>) -> Vec<(i32, i32)>
// {
//     points.sort_by_key(|s|s[0]);
//     let mut x_max = points.iter().max_by_key(|m| m[0]).unwrap()[0];
//     let mut ok_points : HashSet<Vec<i32>> = HashSet::new();
//     for (i, p) in points.iter().enumerate()
//     {
//         let next = i+1;
//         if next < points.len()
//         {
//             let mut incr = 1;
//             while incr <= x_max + 1
//             {
//                 if (p[0] + incr) == points[next][0]
//                 {
//                     ok_points.insert(p.clone());
//                     ok_points.insert(points[next].clone());
//                 }
//                 incr += 1;
//             }
//         }
//     }
//     let mut p: Vec<(i32, i32)> = Vec::with_capacity(ok_points.len());
//     p.extend(ok_points.into_iter().map(|m|(m[0], m[1])));
//     p.sort_by_key(|s|s.0);
//     p
// }

// fn calc_line(points: &mut Vec<Vec<i32>>) -> Vec<(i32, i32)>
// {
//     let mut ok_points : HashSet<Vec<i32>> = HashSet::new();
//     for (i, p) in points.iter().enumerate()
//     {
//         let next = i+1;
//         if next < points.len()
//         {
//             if (p[0]) == points[next][0]
//             {
//                 ok_points.insert(p.clone());
//                 ok_points.insert(points[next].clone());
//             }
//         }
//     }
//     let mut p: Vec<(i32, i32)> = Vec::with_capacity(ok_points.len());
//     p.extend(ok_points.into_iter().map(|m|(m[0], m[1])));
//     p.sort_by_key(|s|s.1);
//     p
// }

// fn solution(points: Vec<Vec<i32>>) -> i32
// {
//     let mut points = points;
//     let y = calc_dia(&mut points);
//     let mut x_count = 1;
//     let mut counts: Vec<u32> = vec![];
//     for (i, p) in y.iter().enumerate()
//     {
//         let next = i+1;
//         if next < y.len()
//         {
//             if (p.1 + 1) == y[next].1 || (p.1 - 1) == y[next].1 || p.1 == y[next].1
//             {
//                 x_count += 1;
//             }
//             else 
//             {
//                 counts.push(x_count);
//                 x_count = 1;
//             }
//         }
//     }
//     counts.push(x_count);
//     x_count = 1;
//     let x_line = calc_line(&mut points);
//     for (i, p) in x_line.iter().enumerate()
//     {
//         let next = i+1;
//         if next < x_line.len()
//         {
//             if (p.1 + 1) == x_line[next].1 || (p.1 - 1) == x_line[next].1
//             {
//                 x_count += 1;
//             }
//             else 
//             {
//                 counts.push(x_count);
//                 x_count = 1;
//             }
//         }
//     }
//     counts.push(x_count);
//     let cnt = counts.iter().max().unwrap();
//     //println!("long  {}", cnt);
//     *cnt as i32
// }


// mod first_not_correct
// {


// fn calc_dia(points: &mut Vec<Vec<i32>>) -> Vec<(i32, i32)>
// {
//     points.sort_by_key(|s|s[0]);
//     let mut ok_points : HashSet<Vec<i32>> = HashSet::new();
//     for (i, p) in points.iter().enumerate()
//     {
//         let next = i+1;
//         if next < points.len()
//         {
//             if (p[0] + 1) == points[next][0]
//             {
//                 ok_points.insert(p.clone());
//                 ok_points.insert(points[next].clone());
//             }
//         }
//     }
//     let mut p: Vec<(i32, i32)> = Vec::with_capacity(ok_points.len());
//     p.extend(ok_points.into_iter().map(|m|(m[0], m[1])));
//     p.sort_by_key(|s|s.0);
//     p
// }

// fn calc_line(points: &mut Vec<Vec<i32>>) -> Vec<(i32, i32)>
// {
//     let mut ok_points : HashSet<Vec<i32>> = HashSet::new();
//     for (i, p) in points.iter().enumerate()
//     {
//         let next = i+1;
//         if next < points.len()
//         {
//             if (p[0]) == points[next][0]
//             {
//                 ok_points.insert(p.clone());
//                 ok_points.insert(points[next].clone());
//             }
//         }
//     }
//     let mut p: Vec<(i32, i32)> = Vec::with_capacity(ok_points.len());
//     p.extend(ok_points.into_iter().map(|m|(m[0], m[1])));
//     p.sort_by_key(|s|s.1);
//     p
// }

// fn solution(points: Vec<Vec<i32>>) -> i32
// {
//     let mut points = points;
//     let y = calc_dia(&mut points);
//     let mut x_count = 1;
//     let mut counts: Vec<u32> = vec![];
//     for (i, p) in y.iter().enumerate()
//     {
//         let next = i+1;
//         if next < y.len()
//         {
//             if (p.1 + 1) == y[next].1 || (p.1 - 1) == y[next].1 || p.1 == y[next].1
//             {
//                 x_count += 1;
//             }
//             else 
//             {
//                 counts.push(x_count);
//                 x_count = 1;
//             }
//         }
//     }
//     counts.push(x_count);
//     x_count = 1;
//     let x_line = calc_line(&mut points);
//     for (i, p) in x_line.iter().enumerate()
//     {
//         let next = i+1;
//         if next < x_line.len()
//         {
//             if (p.1 + 1) == x_line[next].1 || (p.1 - 1) == x_line[next].1
//             {
//                 x_count += 1;
//             }
//             else 
//             {
//                 counts.push(x_count);
//                 x_count = 1;
//             }
//         }
//     }
//     counts.push(x_count);
//     let cnt = counts.iter().max().unwrap();
//     //println!("long  {}", cnt);
//     *cnt as i32
// }
// }


#[cfg(test)]
mod tests
{
    use crate::merge_ksorted_list::ListNode;
    #[test]
    fn test_1()
    {
        let points = vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]];
        println!("{:?}", super::max_points(points));
    }
    #[test]
    fn test_2()
    {
        let points = vec![vec![1,1],vec![2,2],vec![3,3],vec![5,5],vec![6,6],vec![7,7],vec![8,8]];
        println!("{:?}", super::max_points(points));
    }
    #[test]
    fn test_3()
    {
        let points = vec![vec![-1,-1],vec![0,0],vec![1,1], vec![3,3]];
        println!("{:?}", super::max_points(points));
    }
    #[test]
    fn test_3_3()
    {
        let points = vec![vec![1,1],vec![2,1],vec![3,1], vec![4,1]];
        println!("{:?}", super::max_points(points));
    }
    #[test]
    fn test_4()
    {
        let points = vec![vec![-1,-1],vec![0,0],vec![1,1], vec![3,4],vec![3,5],vec![3,6],vec![3,7],vec![3,8]];
        println!("{:?}", super::max_points(points));
    }
    #[test]
    fn test_5()
    {
        let points = vec![vec![-1,-1],vec![0,0],vec![2,2]];
        println!("{:?}", super::max_points(points));
    }
}