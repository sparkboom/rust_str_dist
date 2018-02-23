use std::mem::replace;
use std::ops::{Range, Index, IndexMut};
use std::clone::Clone;
use std::vec::Vec;
use std::fmt::{Debug, Display, Error, Formatter};
use std::result::Result;
use std::iter::repeat;


/// # Distance Matrix (dist_matrix)
/// Is a simple matrix structure that can hold generic value of type T
/// This matrix can have arbitrary indicies that do not have to be 0-based
/// There are also convinience methods to populate rows, cols,or specific values of the matrix.
/// 
/// ## Supported Traits
/// * Index
/// * MutIndex
/// * Debug
pub type Rng = Range<i32>;

pub struct DistMatrix<T>
{
    pub rangex:Rng,
    pub rangey:Rng,
    m:Vec<Vec<T>>
}

impl<T> DistMatrix<T> where T: Clone + Copy + Display
{
    pub fn new(rx:Rng, ry:Rng, default_val:T) -> DistMatrix<T> {
        let rx = rx;
        let ry = ry;
        let w = (rx.start - rx.end).abs() as usize;
        let h = (ry.start - ry.end).abs() as usize;
        DistMatrix { 
            rangex:rx,
            rangey:ry,
            m:vec![vec![default_val; w]; h ]
        }
    }

    pub fn fill(&mut self, rx:&Rng, ry:&Rng, gen:&mut Iterator<Item=T>)
    {
        for x in rx.clone()
        {
            for y in ry.clone()
            {
                let dx = x - self.rangex.start;
                let dy = y - self.rangey.start;
                if let Some(val) = gen.next() {
                    replace(&mut self.m[dy as usize][dx as usize], val);
                } else {
                    return;
                }
                
            }
        }
    }

    pub fn width(&self) -> i32{
        (self.rangex.start - self.rangex.end).abs()
    }

    pub fn height(&self) -> i32{
        (self.rangey.start - self.rangey.end).abs()
    }

    pub fn get_last(&self) -> &T{
        let dx = (self.width()-1) as usize;
        let dy = (self.height()-1) as usize;
        &self.m[dy][dx]
    }

    pub fn row_to_string(&self, y:i32) -> String {

        let s = self.rangex.clone()
            .map(|x| {format!("{:2} ", self[(x, y)])})
            .fold(String::new(), |acc, s| { acc + s.as_str() });

        s
    }
}

/// Index Traits
impl<T> Index<(usize, usize)> for DistMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T  {
        let dx = ((index.0 as i32) - self.rangex.start) as usize;
        let dy = ((index.1 as i32) - self.rangey.start) as usize;
        &self.m[dy][dx]
    }
}
impl<T> Index<(i32, i32)> for DistMatrix<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &T  {
        let dx = (index.0 - self.rangex.start) as usize;
        let dy = (index.1 - self.rangey.start) as usize;
        &self.m[dy][dx]
    }
}
impl<T> IndexMut<(usize, usize)> for DistMatrix<T> {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut T{
        let dx = ((index.0 as i32) - self.rangex.start) as usize;
        let dy = ((index.1 as i32) - self.rangey.start) as usize;
        self.m[dy].get_mut(dx).unwrap()
    }
}
impl<T> IndexMut<(i32, i32)> for DistMatrix<T> {
    fn index_mut<'a>(&'a mut self, index: (i32, i32)) -> &'a mut T{
        let dx = (index.0-self.rangex.start) as usize;
        let dy = (index.1-self.rangey.start) as usize;
        self.m[dy].get_mut(dx).unwrap()
    }
}

/// Debug
impl<T> Debug for DistMatrix<T> where T: Clone + Copy + Display{

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        let out_str = String::new();
        let o = self.rangey.clone()
            .map(|i2| { format!("{}\n", self.row_to_string(i2)) } )
            .collect::<String>();

        write!(f, "\n{}", o)
    }
}

#[cfg(test)]
mod dist_matrix_tests
{
    use super::*;
    
    #[test]
    fn construction_0_2x0_2() {
        let d:DistMatrix<i32> = DistMatrix::new(0..2, 0..2, 0);
        assert_eq!(d.width(), 2);
        assert_eq!(d.height(), 2);
    }
    
    #[test]
    fn construction_0_3x0_3() {
        let d:DistMatrix<i32> = DistMatrix::new(0..3, 0..3, 0);
        assert_eq!(d.width(), 3);
        assert_eq!(d.height(), 3);
    }
    
    #[test]
    fn construction_n1_p1x_n1_p1() {
        let d:DistMatrix<i32> = DistMatrix::new(-1..1,-1..1, 0);
        assert_eq!(d.width(), 2);
        assert_eq!(d.height(), 2);
    }
    
    #[test]
    fn fill_top_row() {
        let mut d:DistMatrix<i32> = DistMatrix::new(0..2,0..2, 0);
        d.fill(&(0..2), &(0..1), &mut repeat(1));

        assert_eq!(d[(0,0)], 1);
        assert_eq!(d[(1,0)], 1);
        assert_eq!(d[(0,1)], 0);
        assert_eq!(d[(1,1)], 0);
    }
    
    #[test]
    fn fill_left_col() {
        let mut d:DistMatrix<i32> = DistMatrix::new(0..2,0..2, 0);
        d.fill( &(0..1), &(0..2), &mut repeat(1));

        assert_eq!(d[(0,0)], 1);
        assert_eq!(d[(1,0)], 0);
        assert_eq!(d[(0,1)], 1);
        assert_eq!(d[(1,1)], 0);
    }
}