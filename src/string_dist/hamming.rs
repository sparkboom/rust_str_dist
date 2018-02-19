use super::helpers::{StringHelpers};

/// Hamming Distance - calculates the string distance using the Hamming Distance algorithm.
/// The Hamming Distance is one of the more simple algorithms, it assumes strings are of the same
/// length. This algorithm is typically inappropriate for text strings that are typed.
/// Mostly used for binary numbers and to measure communication errors.
/// 
/// See - https://en.wikipedia.org/wiki/Hamming_distance
/// 
/// # Example
/// 
/// ```
/// use string_dist::dl;
/// 
/// let str1:String = String.from("anthropology");
/// let str2:String = String.from("anthropophagi");
/// let dist:u8 = hamming::distance(&str1, &str2);
/// println!("The Hamming Distance between {} and {} is {}", str1, str2, dist);
/// ```
/// 
/// * `str1` - The first string to compare
/// * `str2` - The Second string to compare
/// -> The Hamming Distance of the passed 2 strings
pub fn hamming_distance(str1:&str, str2:&str) -> usize
{
    let str_len_delta:usize = (str1.char_count()-str2.char_count()).abs() as usize;

    let (shortest, longest) = if str1.char_count() < str2.char_count() {(str1, str2)} else {(str2, str1)};

    let mut delta:usize = 0;

    for (i, ch1) in shortest.chars().enumerate()
    {
        delta += if ch1 != longest.nth_char(i as i32) {1} else {0};
    }

    (delta + str_len_delta)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn empty_distance() {
        let d = hamming_distance("", "");
        assert_eq!(d, 0);
    }

    #[test]
    fn peter_pedro_distance() {
        let d = hamming_distance("peter", "pedro");
        assert_eq!(d, 3);
    }
}