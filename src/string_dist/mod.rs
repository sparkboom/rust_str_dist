
//
//http://users.cecs.anu.edu.au/~Peter.Christen/publications/tr-cs-06-02.pdf
// Sift3  https://siderite.blogspot.com/2007/04/super-fast-and-accurate-string-distance.html
// Sift4
// Optimized version
// LCS https://en.wikipedia.org/wiki/Longest_common_substring_problem
// diagram n-grams
// https://www.joyofdata.de/blog/comparison-of-string-distance-algorithms/
// Qwerty Keyboard Distance
// Phoenetic Distances
// https://hpi.de/fileadmin/user_upload/fachgebiete/naumann/folien/SS13/DPDC/DPDC_12_Similarity.pdf

// Helpers
mod helpers;

// Edit-Based Simularities
mod jaro;
mod hamming;

pub use self::hamming::hamming_distance;
pub use self::jaro::jaro_simularity;
pub use self::jaro::jaro_winkler_simularity;

pub mod osa; // used for dl
pub mod levenshtein;
pub mod dl;

// Smith-Waterman
// Smith-Waterman-Gotoh




//    Token-Based Simularities

//Words / n-grams
pub mod lcs;
// Jaccard
// Dice
// Cosine Simularity




// Hybrid
//
// Monge-Elkan
// Soft TF-IDF




// Phonetic
//
// Soundex
// Kölner Phonetik
// Metaphone
// Double Metaphone 



// Domain-Dependent
// Dates
// Numerical attributes
// Rules


