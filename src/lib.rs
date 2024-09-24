mod median_sorted_array;
mod string_to_number;
mod divide_without_divide;
mod longest_string_without_repeating;
mod longest_palindrome;
mod reverse_integer;
mod mul_strings;
mod divide_without_div;
mod trapping_rain_water;
mod first_missing_positive;
mod merge_ksorted_list;
mod max_points_on_a_line;
mod extra_characters_in_string;
mod longest_common_prefix;

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
