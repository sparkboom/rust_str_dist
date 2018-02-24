use std::cmp::{max};

/// String Helpers
pub trait StringHelpers<'a>
{
    fn char_count(&'a self) -> i32;
    fn nth_char(&'a self, x:i32) -> char;
}
impl<'a> StringHelpers<'a> for &'a str
{
    fn char_count(&'a self) -> i32 {
        self.chars().count() as i32
    }
    fn nth_char(&'a self, x:i32) -> char {
        self.chars().nth(x as usize).unwrap()
    }
}

/// # Distance - To - Simularity
/// Converts a distance metric to a simularity value.
/// Simularity values range from 0.0 to 1.0. This is useful to normalize 
/// best matches with other distances.
/// 
/// ## Complexity
/// Where s1 is the length of str1, and s2 is the length of str2
/// * Time: s1+s2
/// * Space: minimal
/// 
/// ## Parameters
/// * `str1` - The first string to compare
/// * `str2` - The Second string to compare
/// * -> The simularity value. 0.0 <= value <= 1.0
pub fn distance_to_simularity(str1:&str, str2:&str, dist:f64) -> f64
{
    let longest = max(str1.char_count(), str2.char_count());
    let m = longest as f64;
    if m == 0.0 {
        return 1.0;
    }
    (1.0 - (dist / m)) as f64
}