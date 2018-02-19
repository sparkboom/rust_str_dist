use helpers::print_matrix;
use helpers::min3;
use std::cmp;

/**
The Optimal String-Alignment Distance, is a variation of the Damerau-Levenshtein distance
that returns the strings' edit distance - account for deletion, insertion substitution, and 
transposition under the condition that no substring is edited more than once.
*/
pub fn distance(str1:&str, str2:&str) -> u8
{
    let mut score:u8 = 0;
    let mut vy:Vec<Vec<u8>> = Vec::new();

    let char_count:u8 = str1.chars().count() as u8;
    vy.push( (0u8..char_count+1).collect() );

    for (y, ch2) in str2.char_indices()
    {
        let mut vx:Vec<u8> = Vec::new();
        vx.push((y as u8) + 1);

        for (x, ch1) in str1.char_indices()
        {
            let cost:u8 = if ch1 == ch2 {0} else {1};
            score = min3( vx[x]+1 , vy[y][x+1]+1 , vy[y][x]+cost );

            if x > 0 && y > 0 {
                let prev_ch1 = str1.chars().nth(x-1).unwrap();
                let prev_ch2 = str2.chars().nth(y-1).unwrap();

                if prev_ch2 == ch1 && prev_ch1 == ch2 {
                    score = cmp::min( score, vy[y-1][x-1] + cost);
                }
            }
            vx.push(score);
        }
        vy.push(vx);
    }

    println!("OSA Distance Calc:");
    print_matrix(str1, str2, &vy);
    score
}