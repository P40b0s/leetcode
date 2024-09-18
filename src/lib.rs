mod median_sorted_array;
mod string_to_number;
mod divide_without_divide;
mod longest_string_without_repeating;
mod longest_palindrome;
mod reverse_integer;
mod mul_strings;
mod divide_without_div;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
