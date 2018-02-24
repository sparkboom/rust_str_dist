mod dist_matrix;

pub mod levenshtein
{
    use std::iter::repeat;
    use std::ops::Range;
    use std::cmp::{min, max};
    use std::collections::HashMap;
    use super::super::helpers::StringHelpers;
    
    use super::dist_matrix::DistMatrix;

    struct CalcState<'a, T: 'a> {
        matrix:&'a DistMatrix<T>,
        cost: i32,
        score: i32,
        i1: i32,
        i2: i32,
        str1: &'a str,
        str2: &'a str
    }
    type CalcScoreFn<T> = Fn( &CalcState<T> ) -> T;

    /// # Levenshtein Distance 
    /// Calculates the string distance using the Levenshtein algorithm.
    /// It is also defined as minimum number of simple edit operations on string to change it into another.
    /// Allowed operations are deletion, insertion and substitution of a single character.
    /// 
    /// See - https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
    /// 
    /// ## Use
    /// - Typically used to compare strings of similar length
    /// 
    /// ## Example
    /// ```
    /// let str1:String = String.from("anthropology");
    /// let str2:String = String.from("anthropophagi");
    /// let dist:u8 = levenshtein_distance(&str1, &str2);
    /// println!("The Levenshtein Distane between {} and {} is {}", str1, str2, dist);
    /// ```
    /// 
    /// ## Complexity
    /// - Time:  O(|str1.len|, |str2.len|)       (to fill matrix)
    /// - Space: O(min(|str1.len|, |str2.len|))  (improvment - the algorithm can be improved to use only 2 rows)
    /// - Other Cost Models:
    ///     insert & delete cost 1.0, replacement cost 0.5   (where change in string length is punished more)
    ///     char-based costs
    ///         OCR has simularities - m-n, 1-l-i
    ///         Keyboard has closeness of keys (local specific) - a-s, s-d, q-w, w-e
    ///         Mind may associate characters (subjective) - 6-9
    ///         Biology such as genetic pairs - a-t
    /// 
    /// ## Parameters
    /// * `str1` - The first string to compare
    /// * `str2` - The Second string to compare
    /// -> The Levenshtein Distance of the passed 2 strings, 0 <= distance <= max(|str1.len|, |str2.len|)
    pub fn levenshtein_distance(str1:&str, str2:&str) -> usize
    {
        let calc_score = | state:&CalcState<usize> | { state.score as usize };
        let matrix = build_levenshtein_matrix(str1, str2, &calc_score );
        println!("Levenshtein Calc:\n{:?}\n", matrix);
        *matrix.get_last()
    }
    #[cfg(test)]
    mod levenshtein_distance_tests {
        use super::*;
        
        #[test]
        fn empty_distance() {
            let d = levenshtein_distance("", "");
            assert_eq!(d, 0);
        }

        #[test]
        fn peter_pedro_distance() {
            let d = levenshtein_distance("jones", "johnson");
            assert_eq!(d, 4);
        }

        #[test]
        fn paul_pual_distance() {
            let d = levenshtein_distance("paul", "pual");
            assert_eq!(d, 2);
        }

        #[test]
        fn paul_jones_distance() {
            let d = levenshtein_distance("Paul Jones", "Jones, Paul");
            assert_eq!(d, 11);
        }
    }

    pub fn levenshtein_simularity(str1:&str, str2:&str) -> f64
    {
        let m = max(str1.char_count(), str2.char_count()) as f64;
        if m == 0.0 {
            return 1.0;
        }
        let d = levenshtein_distance(str1, str2) as f64;
        1.0 - (d / m)
    }
    #[cfg(test)]
    mod levenshtein_simularity_tests {
        use super::*;
        
        #[test]
        fn empty_simularity() {
            let d = levenshtein_simularity("", "");
            assert_eq!(d, 1.0);
        }

        #[test]
        fn peter_pedro_simularity() {
            let d = levenshtein_simularity("jones", "johnson");
            let expected = 0.43;
            assert!((expected - d) < 0.01, "The expected value {} and actual value {} are not close enough", d, expected);
        }

        #[test]
        fn paul_pual_simularity() {
            let d = levenshtein_simularity("paul", "pual");
            assert_eq!(d, 0.5);
        }

        #[test]
        fn paul_jones_simularity() {
            let d = levenshtein_simularity("Paul Jones", "Jones, Paul");
            assert_eq!(d, 0.0);
        }
    }

    /// # Optimal String Alignment Distance (or Restricted Edit Distance)
    /// Calculates the string distance using the OSA Distance algorithm.
    /// It is also defined as minimum number of simple edit operations on string to change it into another, but the list of allowed operations is extended.
    /// There are 4 allowed edits: deletion, insertion and substitution of a single character and an transposition of two adjacent characters.
    /// Each substring may only be edited once.
    /// 
    /// See - https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
    /// See - https://www.guyrutenberg.com/2008/12/15/damerau-levenshtein-distance-in-python/
    /// 
    /// ## Use
    /// - 
    /// 
    /// ## Example
    /// 
    /// ```
    /// let str1:String = String.from("anthropology");
    /// let str2:String = String.from("anthropophagi");
    /// let dist:u8 = osa_distance(&str1, &str2);
    /// println!("The OSA Distance between {} and {} is {}", str1, str2, dist);
    /// ```
    /// 
    /// ## Complexity
    /// - Time:  O(|str1.len|, |str2.len|)       (to fill matrix)
    /// - Space: O(min(|str1.len|, |str2.len|))  (improvment - the algorithm can be improved to use only 2 rows)
    /// 
    /// ## Parameters
    /// * `str1` - The first string to compare
    /// * `str2` - The Second string to compare
    /// -> The OSA Distance of the passed 2 strings
    pub fn osa_distance<'a>(str1:&'a str, str2:&'a str) -> usize
    {
        let calc_score = | s:&CalcState<usize> | { 

            if  s.i1>0 && s.i2>0 && 
                s.str1.nth_char(s.i1) == s.str2.nth_char(s.i2-1) &&
                s.str1.nth_char(s.i1-1) == s.str2.nth_char(s.i2) {
                return min(s.score as usize, s.matrix[(s.i1-2,s.i2-2)] + s.cost as usize );
            } else {
                return s.score as usize;
            }
        };
        let matrix = build_levenshtein_matrix(str1, str2, &calc_score );

        println!("OSA Calc:\n{:?}\n", matrix);
        *matrix.get_last()
    }
    #[cfg(test)]
    mod osa_distance_tests {
        use super::*;
        
        #[test]
        fn empty_distance() {
            let d = osa_distance("", "");
            assert_eq!(d, 0);
        }

        #[test]
        fn peter_pedro_distance() {
            let d = osa_distance("jones", "johnson");
            assert_eq!(d, 4);
        }

        #[test]
        fn paul_pual_distance() {
            let d = osa_distance("paul", "pual");
            assert_eq!(d, 1);
        }

        #[test]
        fn paul_jones_distance() {
            let d = osa_distance("Paul Jones", "Jones, Paul");
            assert_eq!(d, 11);
        }
    }

    /// # Damerau Levenshtein Distance 
    /// Calculates the string distance using the Damerau Levenshtein algorithm.
    /// It is also defined as minimum number of simple edit operations on string to change it into another, but the list of allowed operations is extended.
    /// There are 4 allowed edits: deletion, insertion and substitution of an single character and an transposition of two adjacent characters.
    /// Similar to Optimal String Alignment, but substrings can be eidted for an unlimited no. of times. The triangle inequality holds, therefore, the 
    /// value produced by this function is considered a real distance.
    ///
    /// 
    /// See - https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
    /// See - https://www.guyrutenberg.com/2008/12/15/damerau-levenshtein-distance-in-python/
    /// 
    /// ## Use
    /// - More natural spell checking
    /// - Natural Language Processing
    /// - DNA - undergoes these 4 edits in nature
    /// - Protein Sequence differences
    /// - Fraud Detection
    /// 
    /// ## Example
    /// 
    /// ```
    /// let str1:String = String.from("anthropology");
    /// let str2:String = String.from("anthropophagi");
    /// let dist:u8 = damerau_levenshtein_distance(&str1, &str2);
    /// println!("The Damerau Levenshtein Distane between {} and {} is {}", str1, str2, dist);
    /// ```
    /// 
    /// ## Complexity
    /// 
    /// ## Parameters
    /// * `str1` - The first string to compare
    /// * `str2` - The Second string to compare
    /// -> The Damerau Levenshtein Distance of the passed 2 strings
    pub fn damerau_levenshtein_distance(str1:&str, str2:&str) -> usize
    {
        if str1 == str2 {return 0;}

        let r1 = -1..(str1.chars().count()+1)as i32;
        let r2 = -1..(str2.chars().count()+1)as i32;

        let bounds = |r:&Range<i32>| { (r.end-r.start).abs() as usize };

        // leave if either string is empty
        if bounds(&r1) < 2 {return bounds(&r2);}
        if bounds(&r2) < 2 {return bounds(&r1);}

        let mut m:DistMatrix<usize> = DistMatrix::new(r1.clone(), r2.clone() ,0);
        let max_dist = bounds(&r1) + bounds(&r2) - 4;

        m.fill( &(-1..0), &r2, &mut repeat(max_dist));
        m.fill( &r1, &(-1..0), &mut repeat(max_dist));
        m.fill( &(0..r1.end), &(0..1), &mut (0..));
        m.fill( &(0..1), &(0..r2.end), &mut (0..));
        
        let mut chars: HashMap<char, usize> = HashMap::new();
        
        for (i1, ch1) in str1.chars().enumerate()
        {
            let mut db:i32 = -1;

            for (i2, ch2) in str2.chars().enumerate()
            {
                let k:i32 = if let Some(v) = chars.get(&ch2) {*v as i32} else{-1};
                let l = db;

                let mut cost = 1;
                if ch1 == ch2 {
                    cost = 0;
                    db = i2 as i32; 
                }

                let score = {
                    let subst = m[(i1, i2)] + cost;
                    let insert = m[(i1, i2+1)] + 1;
                    let del = m[(i1+1, i2)] + 1;
                    let transp = m[(k,l)] + ((i1 as i32) - k - 1) as usize + ((i2 as i32) - l - 1) as usize + 1;

                    min!(subst, insert, del, transp)
                };

                m[(i1+1,i2+1)] = score;
            }

            chars.insert(str1.nth_char(i1 as i32), i1);
        }

        println!("{:?}", m);
        *m.get_last()
    }
    #[cfg(test)]
    mod damerau_levenshtein_distance_tests {
        use super::*;
        
        #[test]
        fn empty_distance() {
            let d = damerau_levenshtein_distance("", "");
            assert_eq!(d, 0);
        }

        #[test]
        fn peter_pedro_distance() {
            let d = damerau_levenshtein_distance("jones", "johnson");
            assert_eq!(d, 4);
        }

        #[test]
        fn paul_pual_distance() {
            let d = damerau_levenshtein_distance("paul", "pual");
            assert_eq!(d, 1);
        }

        #[test]
        fn paul_jones_distance() {
            let d = damerau_levenshtein_distance("Paul Jones", "Jones, Paul");
            assert_eq!(d, 11);
        }
    }


    fn build_levenshtein_matrix<'a>(str1:&'a str, str2:&'a str, calc_score: &'a CalcScoreFn<usize>) -> DistMatrix<usize>{
        let mut m:DistMatrix<usize> = DistMatrix::new(-1..str1.char_count(), -1..str2.char_count(), 0);
        m.fill( &(-1..0), &(-1..str2.char_count()), &mut (0..));
        m.fill( &(-1..str1.char_count()), &(-1..0), &mut (0..));

        for i1 in 0..str1.char_count()
        {

            for i2 in 0..str2.char_count()
            {
                let score = {

                    let ch1 = str1.nth_char(i1);
                    let ch2 = str2.nth_char(i2);
                    let cost:i32 = if ch1 == ch2 {0} else {1};
                    
                    let score:i32 = {    
                        let del = m[(i1-1,i2)] + 1;
                        let insert = m[(i1,i2-1)] + 1;
                        let subst = m[(i1-1,i2-1)] + cost as usize;
                        min!( del, insert, subst) as i32
                    };
                    
                    let state:CalcState<usize> = CalcState { matrix:&m, cost:cost, score: score, i1:i1, i2:i2, str1:str1, str2:str2  };
                    calc_score(&state)
                };
                m[(i1,i2)] = score;
            }
        }

        m
    }

    /*
    Distance_Matrix
    // use arrays of fixed length,
    // Support common traits
    // Support default values
    // Support assignment operations for whole row or column with fixed or incremental values



    Compact_Distance_Matrix
    // used if poss
    // only stores 2 rows at a time

    */
}