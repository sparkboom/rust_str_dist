
use std::cmp::min;

pub fn cnt(s:&str) -> usize { s.chars().count() }
pub fn min3(a:u8, b:u8, c:u8) -> u8 { min( min( a, b ), c ) }

pub struct StringComparer<'a> {
    str1:&'a str,
    str2:&'a str,
    matrix:Vec<Vec<u8>>
}

impl<'a> StringComparer<'a> {
    fn new(str1:&'a str, str2:&'a str) -> StringComparer<'a> {
        let mut matrix:Vec<Vec<u8>> = Vec::new();
        let char_count:u8 = str1.chars().count() as u8;
        matrix.push((0u8..char_count+1).collect());
        for y in 0..str2.chars().count()
        {
            matrix.push(vec![(y + 1) as u8]);
        }
        (StringComparer{ str1:str1, str2:str2, matrix:matrix })
    }
}

pub fn print_matrix(str1:&str, str2:&str, m:&Vec<Vec<u8>>){

    print!("     ");
    for ch in str1.chars()
    {
        print!("{}  ",ch);
    }
    print!("\n");
    let full_str = format!(" {}", str2);
    let mut str_iter = full_str.chars();
    for line in m.iter()
    {
        if let Some(ch) = str_iter.next() { print!("{} ", ch); }
        for score in line.iter()
        {
            print!("{:02} ", score);
        }
        print!("\n");
    }
    print!("\n");
}
