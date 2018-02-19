use super::helpers::{StringHelpers};
use std::cmp::min;
use std::cmp::max;
use std::str;
use std::f64;

/// The standard value for this constant is 0.1 in Winkler's work.
static DEFAULT_SCALING_FACTOR:f64 = 0.1;

pub struct JWOptions 
{
    scaling_factor:Option<f64>,
    case_sensitive:bool
}

/// The Jaro Simularity - calculates the string distance. Often referred as a distance metric, 
/// 
/// See - https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
/// See - https://hpi.de/fileadmin/user_upload/fachgebiete/naumann/folien/SS13/DPDC/DPDC_12_Similarity.pdf
/// 
/// # Example
/// 
/// ```
/// let str1:String = String.from("anthropology");
/// let str2:String = String.from("anthropophagi");
/// let dist:f64 = jaro_simularity&str1, &str2);
/// println!("The Jaro Distance between {} and {} is {}", str1, str2, dist);
/// ```
/// 
/// * `str1` - The first string to compare
/// * `str2` - The Second string to compare
/// * `opts` - Options to pass to vary the calculation weights.
/// 
/// -> f64 The Jaro Winkler Distance of the passed 2 strings. A double value between 0 and 1. 0 meaning no simularity, and 1 being identical.
pub fn jaro_simularity(str1:&str, str2:&str, opts:Option<JWOptions>) -> f64
{
    let (len1, len2) = (str1.char_count() as f64, str2.char_count() as f64);

    // Leave early as at least 1 string is empty
    if len1 == 0f64 && len2 == 0f64 { return 1f64; }
    if len1 == 0f64 || len2 == 0f64 { return 0f64; }

    // Use lowercase if case insensitive
    let lcs1 = str1.to_string().to_lowercase();
    let lcs2 = str2.to_string().to_lowercase();

    let is_case_sens = |o:&Option<JWOptions>| {
        if let Some( JWOptions {case_sensitive: cs, ..} ) = *o {
            return cs;
        } 
        return false;
    };

    let (s1,s2) = if is_case_sens(&opts) { (str1, str2) } else { (lcs1.as_str(), lcs2.as_str()) };
    
    // Leave early if strings are identical
    if s1 == s2 { return 1f64; }

    let match_dist = ((f64::max(len1, len2)/2.0).floor() - 1.0) as i32;
    let mut s1_matches:Vec<bool> = vec![false; len1 as usize]; 
    let mut s2_matches:Vec<bool> = vec![false; len2 as usize]; 
    let mut m:f64 = 0.0;

    for i in 0..len1 as i32
    {
        let low  = max(i-match_dist, 0);
        let high = min(i+match_dist,len2 as i32 - 1);

        for j in low..high+1
        {
            if s2_matches[j as usize] == false && s1.nth_char(i) == s2.nth_char(j) {
                m += 1.0;
                s1_matches[i as usize] = true;
                s2_matches[j as usize] = true;
                break;
            }
        }
    }

    // Leave early if no matches found
    if m == 0.0 { return 0f64; }

    // Count the transpositions.
    let mut k = 0;
    let mut num_trans:f64 = 0.0;

    for i in 0..len1 as i32
    {
        if s1_matches[i as usize] { 

            let mut j = k;
            while s2_matches[j as usize] == false { k = j + 1; j += 1; }


            if s1_matches[i as usize] != s2_matches[j as usize] {
                num_trans += 0.5;
            }
        }     
    }

    // The Jaro distance
    let weight = (m / len1  + m / len2 + (m - num_trans) / m) / 3.0;

    weight
}

/// The Jaro Winkler Simularity - calculates the string distance 
/// using the variant of the Jaro algorithm proposed by Wrinkler. Often referred as a distance metric.
/// This algorithm gives more favourable ratings to matches at the beginning of the string.
/// 
/// See - https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
/// 
/// Based on Code at http://commons.apache.org/proper/commons-text/jacoco/org.apache.commons.text.similarity/JaroWinklerDistance.java.html
/// And https://github.com/jordanthomas/jaro-winkler/blob/master/index.js
/// 
/// # Example
/// 
/// ```
/// let str1:String = String.from("anthropology");
/// let str2:String = String.from("anthropophagi");
/// let dist:f64 = jaro_winkler_simularity(&str1, &str2);
/// println!("The Jaro Wrinkler Distance between {} and {} is {}", str1, str2, dist);
/// ```
/// 
/// * `str1` - The first string to compare
/// * `str2` - The Second string to compare
/// * `opts` - Options to pass to vary the calculation weights.
/// 
/// -> f64 The Jaro Winkler Distance of the passed 2 strings. A double value between 0 and 1. 0 meaning no simularity, and 1 being identical.
pub fn jaro_winkler_simularity(str1: &str, str2: &str, opts:Option<JWOptions>) -> f64
{

    // Jaro Winkler weight, JW gives more favourable ratings to matches at the beginning of the string
    let p:f64 = if let Some(JWOptions{scaling_factor:Some(sf),..}) = opts {(sf)} else {DEFAULT_SCALING_FACTOR};
    let weight = jaro_simularity(str1, str2, opts);
    let mut jw_weight = weight;
    let mut l = 0;

    if weight == 0.0 || weight == 1.0 {
        return weight;
    }

    let get_char = |s:&str, i:usize|{s.chars().nth(i).unwrap()};
    if weight > 0.7 {
      
      while get_char(str1,l) == get_char(str2,l) && l < 4
      {
        l += 1;
      }

      jw_weight = weight + ((l as f64) * p * (1.0 - weight));
    }

    return jw_weight;
}

/// Jaro Winkler Distance Tests. 
/// 
/// Test data was retrieved from http://commons.apache.org/proper/commons-text/jacoco/org.apache.commons.text.similarity/JaroWinklerDistance.java.html
/// 
#[cfg(test)]
mod tests {
    use super::*;

    static EPSILON:f64 = 0.04;
    fn cmp_approx (actual:f64, expected:f64) -> bool { (actual - expected).abs() < EPSILON }
    
    #[test]
    fn empty_distance() {
        let d = jaro_winkler_simularity("", "", None);
        assert_eq!(d, 1.0f64);
    }

    #[test]
    fn a_to_empty_distance() {
        let d = jaro_winkler_simularity("a", "", None);
        assert_eq!(d, 0.0f64);
    }

    #[test]
    fn aaaapppp_to_empty_distance() {
        let d = jaro_winkler_simularity("aaaapppp", "", None);
        assert_eq!(d, 0.0f64);
    }

    #[test]
    fn frog_fog_distance() {
        let d = jaro_winkler_simularity("frog", "fog", None);
        let expected_d = 0.93f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn fly_ant_distance() {
        let d = jaro_winkler_simularity("fly", "ant", None);
        assert_eq!(d, 0.0f64);
    }

    #[test]
    fn elephant_hippo_distance() {
        let d = jaro_winkler_simularity("elephant", "hippo", None);
        let expected_d = 0.44f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn hippo_elephant_distance() {
        let d = jaro_winkler_simularity("hippo", "elephant", None);
        let expected_d = 0.44f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn hello_hallo_distance() {
        let d = jaro_winkler_simularity("hello", "hallo", None);
        let expected_d = 0.88f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn abc_corp_distance() {
        let d = jaro_winkler_simularity("ABC Corporation", "ABC Corp", None);
        let expected_d = 0.93f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn ampersand_comparison() {
        let d = jaro_winkler_simularity("D N H Enterprises Inc", "D & H Enterprises, Inc.", None);
        let expected_d = 0.95f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn name_abbreviation_comparison() {
        let d = jaro_winkler_simularity("My Gym Children's Fitness Center", "My Gym. Childrens Fitness", None);
        let expected_d = 0.92f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }

    #[test]
    fn typo_comaprison() {
        let d = jaro_winkler_simularity("PENNSYLVANIA", "PENNCISYLVNIA", None);
        let expected_d = 0.92f64;
        assert!(cmp_approx(d, expected_d), format!("Expected value {} and actual value {} were too far apart", expected_d, d));
    }
}
