#![allow(unused_variables)]
#![allow(dead_code)]

mod string_dist;

use string_dist::lcs::distance as lcs;
use string_dist::lcs::distance2 as lcs2;
use string_dist::*;

fn main()
{
    let str1 = "failuree";
    let str2 = "faluiere";

    let hd = hamming_distance( str1, str2 );
    let dld = damerau_levenshtein_distance( str1, str2 );
    let ld = levenshtein_distance( str1, str2 );
    let osa = osa_distance( str1, str2 );
    let j = jaro_simularity( str1, str2, None );
    let jw = jaro_winkler_simularity( str1, str2, None );
    
    let lcs_hash = lcs( str1, str2 );
    let lcs_vec = lcs2( str1, str2 );
    let lcs:u8 = if lcs_hash.len() > 0 {*(lcs_hash.values().next().unwrap())} else {0u8};
    let lcs2:usize = if lcs_vec.len() > 0 {lcs_vec.get(0).unwrap().str.clone().chars().count()} else {0};
    
    // Edit-Based Distances
    println!("Hamming Distance: {}", hd);

    println!("Levenshtein Distance: {}", ld);
    println!("Damerau Levenshtein Distance: {}", dld);
    println!("OSA Distance: {}", osa);

    println!("Jaro Simularity: {}", j);
    println!("Jaro Wrinkler Simularity: {}", jw);



    // Token-Based Distances

    println!("Longest Common Substring Distance: {}", lcs);
    println!("Longest Common Substring Distance2: {}", lcs2);
}