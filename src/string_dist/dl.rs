use std::cmp::min;
use super::helpers::{ScoreMatrix,min3, StringHelpers};

/// Damerau Levenshtein Distance - calculates the string distance using the Damerau Levenshtein algorithm.
/// It is also defined as minimum number of simple edit operations on string to change it into another, but the list of allowed operations is extended.
/// As it is written on Wikipedia there are 4 allowed edits: deletion, insertion and substitution of an single character and an transposition of two adjacent characters.
/// 
/// See - https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
/// See - https://www.guyrutenberg.com/2008/12/15/damerau-levenshtein-distance-in-python/
/// 
/// # Example
/// 
/// ```
/// use string_dist::dl;
/// 
/// let str1:String = String.from("anthropology");
/// let str2:String = String.from("anthropophagi");
/// let dist:u8 = dl::distance(&str1, &str2);
/// println!("The Damerau Levenshtein Distane between {} and {} is {}", str1, str2, dist);
/// ```
/// 
/// * `str1` - The first string to compare
/// * `str2` - The Second string to compare
/// -> The Damerau Levenshtein Distance of the passed 2 strings
pub fn distance(str1:&str, str2:&str) -> usize
{
    let mut matrix = ScoreMatrix::new(str1, str2);
    matrix.init_incrementing_borders();

    for i1 in 0..str1.char_count()
    {
        for i2 in 0..str2.char_count()
        {
            let ch1 = str1.nth_char(i1);
            let ch2 = str2.nth_char(i2);
            let cost:usize = if ch1 == ch2 {0} else {1};

            let val = {    
                let del = matrix[(i1-1,i2)] + 1;
                let insert = matrix[(i1,i2-1)] + 1;
                let subst = matrix[(i1-1,i2-1)] + cost;
                min3(del, insert, subst)
            };

            if i1>0 && i2>0 && matrix.are_chars_equal(i1,i2-1) && matrix.are_chars_equal(i1-1,i2) {
                let transp = matrix[(i1-2,i2-2)];
                matrix[(i1,i2)] = min(val, transp + cost);
            } else {
                matrix[(i1,i2)] = val;
            }
        }
    }

    println!("DL Calc:\n{:?}\n", matrix);
    matrix.last_score()
}
