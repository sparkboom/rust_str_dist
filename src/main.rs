#![allow(unused_variables)]
#![allow(dead_code)]

mod string_dist;
mod helpers;

use string_dist::levenshtein::distance as levenshtein;
use string_dist::osa::distance as osa;
use string_dist::dl::distance as dl;
use string_dist::lcs::distance as lcs;
use string_dist::lcs::distance2 as lcs2;
use string_dist::*;

fn main()
{
    let str1 = "failuree";
    let str2 = "faluiere";

    let dld = dl( str1, str2 );
    let hd = hamming_distance( str1, str2 );
    let ld = levenshtein( str1, str2 );
    let j = jaro_simularity( str1, str2, None );
    let jw = jaro_winkler_simularity( str1, str2, None );
    let osa = osa( str1, str2 );
    
    let lcs_hash = lcs( str1, str2 );
    let lcs_vec = lcs2( str1, str2 );
    let lcs:u8 = if lcs_hash.len() > 0 {*(lcs_hash.values().next().unwrap())} else {0u8};
    let lcs2:usize = if lcs_vec.len() > 0 {lcs_vec.get(0).unwrap().str.clone().chars().count()} else {0};
    
    // Edit-Based Distances
    println!("Hamming Distance: {}", hd);
    println!("OSA Distance: {}", osa);
    println!("Levenshtein Distance: {}", ld);
    println!("Damerau Levenshtein Distance: {}", dld);
    println!("Jaro Wrinkler Simularity: {}", j);
    println!("Jaro Wrinkler Simularity: {}", jw);

    //

    println!("Longest Common Substring Distance: {}", lcs);
    println!("Longest Common Substring Distance2: {}", lcs2);
}