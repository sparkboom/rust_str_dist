#![allow(unused_assignments)]

use std::fmt;
use std::collections::HashMap;

// Longest Common Substring
// LCS https://en.wikipedia.org/wiki/Longest_common_substring_problem

pub fn distance(str1:&str, str2:&str) -> HashMap<String, u8>
{
    let mut substrs:HashMap<String, u8> = HashMap::new();
    let mut vy:Vec<Vec<u8>> = Vec::new();
    let mut z:u8 = 0;
    let mut score:u8 = 0;
    for (y, ch2) in str2.char_indices()
    {
        let mut vx:Vec<u8> = Vec::new();
        for (x, ch1) in str1.char_indices()
        {
            score = 0;
            if ch1 == ch2 {

                score = if x == 0 || y == 0 {1} else {vy[y-1][x-1] + 1};

                if score > z {
                    z = score;
                }else if score == z {
                    let s:String = str1[x+1-(z as usize)..x].to_string();
                    substrs.insert(s, score);
                }

            } else {
                score = 0;
            };
            vx.push(score);
        }
        vy.push(vx);
    }

    println!("Longest Common Substring Calc:");
    println!("{:?}", substrs);
    substrs
}

pub struct Match {
    pub str:String,
    idx1:usize,
    idx2:usize
}
impl fmt::Debug for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ idx1:{}, @ idx2:{}", self.str, self.idx1, self.idx2)
    }
}
pub fn get_deeper_matches(mtchs:&Vec<Match>, str1:&str, str2:&str) -> Vec<Match>
{
    let mut m = Vec::new();
    for mtch in mtchs
    {
        let offset = mtch.str.chars().count();
        if let (Some(ch1), Some(ch2)) = ( str1.chars().nth(mtch.idx1+offset) , str2.chars().nth(mtch.idx2+offset) ){
            if ch1 == ch2 {
                let new_str = mtch.str.clone() + &ch1.to_string();
                m.push( Match{str:new_str, idx1:mtch.idx1, idx2:mtch.idx2 } );
            }
        }
    }
    m
}
pub fn distance2(str1:&str, str2:&str) -> Vec<Match>
{
    let mut matches:HashMap<char, usize> = HashMap::new();
    for (i, ch) in str1.char_indices()
    {
        matches.insert(ch, i);
    }

    let mut m = Vec::new();
    for (i2, ch) in str2.char_indices()
    {
        if let Some(i1) = matches.get(&ch){
            m.push(Match{str:ch.to_string(), idx1:*i1, idx2:i2 });
        }
    }

    let mut n:Vec<Match> = Vec::new();
    while !m.is_empty()
    {
        n = m;
        m = get_deeper_matches(&n, str1, str2);
    }
    
    println!("Longest Common Substring2 Calc:");
    println!("{:?}", n);
    n
}