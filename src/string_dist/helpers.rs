use std::collections::HashMap;
use std::cmp::{Ord,PartialEq};
use std::cmp::min;
use std::fmt::{self, Debug, Result};
use std::ops::{Range, Index, IndexMut};

// String Helpers
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

pub fn min3<T:Ord>(a:T, b:T, c:T) -> T
{ 
    min(min(a,b),c) 
}

// Coordinates - we use a struct unfortunately as tuples cannot be hashed
#[derive(Eq,Clone,Hash,Default)]
struct Coordinate
{
    x:i32,
    y:i32
}
impl Debug for Coordinate
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({},{})", self.x, self.y)
    }
}
impl PartialEq for Coordinate
{
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Score Matrix
pub struct ScoreMatrix<'a>
{
    str1:&'a str,
    str2:&'a str,
    pub range1:Range<i32>,
    pub range2:Range<i32>,
    map:HashMap<Coordinate,usize>
}
impl<'a> ScoreMatrix<'a>
{
    pub fn new(str1:&'a str,str2:&'a str)->ScoreMatrix<'a>{
        ScoreMatrix { 
            str1:str1, 
            str2:str2, 
            map:HashMap::new(), 
            range1:0i32..str1.char_count(), 
            range2:0i32..str2.char_count() 
        }
    }
    pub fn init_incrementing_borders(&mut self){
        for x in -1..self.range1.end+1
        {
            self.map.insert(Coordinate{x:x,y:-1}, (x + 1) as usize);
        }
        for y in -1..self.range2.end+1
        {
            self.map.insert(Coordinate{x:-1,y:y}, (y + 1) as usize);
        }
        self.range1 = -1..self.str1.char_count();
        self.range2 = -1..self.str2.char_count();
    }

    pub fn are_chars_equal(&self, i1:i32, i2:i32) -> bool
    {
        let ch1 = self.str1.chars().nth(i1 as usize);
        let ch2 = self.str2.chars().nth(i2 as usize);

        if let ( Some(c1), Some(c2)) = (ch1, ch2) {
            return c1 == c2;
        }

        return false;
    }

    pub fn last_score(&self) -> usize 
    {
        let c = Coordinate{x:self.range1.end-1,y:self.range2.end-1};
        self.map[&c]
    }
}
type ExtCoordinates = (i32,i32);
type MatrixValue = usize;
impl<'a> Index<ExtCoordinates> for ScoreMatrix<'a> {
    type Output = MatrixValue;

    fn index<'b>(&'b self, index: ExtCoordinates) -> &'b Self::Output {
        let c = Coordinate{x:index.0,y:index.1};
        &self.map[&c]
    }
}
impl<'a> IndexMut<ExtCoordinates> for ScoreMatrix<'a> {
    fn index_mut<'b>(&'b mut self, index: ExtCoordinates) -> &'b mut MatrixValue {
        let c = Coordinate{x:index.0,y:index.1};
        if self.map.get(&c) == None {
            self.map.insert(Coordinate{x:index.0,y:index.1}, 0 as usize);
        }
        self.map.get_mut(&c).unwrap()
    }
}
impl<'a> Debug for ScoreMatrix<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        let mut out_str = String::from("    ");
        for i in  self.range1.clone()
        {
            if i<0 {
                out_str += "   ";
            } else {
                out_str = format!("{}{}  ", out_str, self.str1.nth_char(i) );
            }
        }
        out_str.push('\n');

        for i2 in self.range2.clone()
        {
            if i2<0 {
                out_str += "   ";
            } else {
                out_str = format!("{}{}  ", out_str, self.str2.nth_char(i2));
            }
            for i1 in self.range1.clone()
            {
                out_str = format!("{}{:2} ", out_str, self[(i1, i2)]);
            }
            out_str.push('\n');
        }

        write!(f, "{}", out_str)
    }
}