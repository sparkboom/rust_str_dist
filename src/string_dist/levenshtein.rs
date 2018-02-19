use helpers::print_matrix;
use helpers::min3;

/*
This is a measure of the strings' edit distance - taking into account
deletion, insertion and substitution
*/
pub fn distance(str1:&str, str2:&str) -> u8
{
    let mut score:u8 = 0;
    let mut vy:Vec<Vec<u8>> = Vec::new();

    let char_count:u8 = str1.chars().count() as u8;
    vy.push((0u8..char_count+1).collect());
    for (y, ch2) in str2.char_indices()
    {
        let mut vx:Vec<u8> = Vec::new();
        vx.push((y as u8) + 1);
        for (x, ch1) in str1.char_indices()
        {
            let cost:u8 = if ch1 == ch2 {0} else {1};
            score = min3( vx[x]+1 , vy[y][x+1]+1 , vy[y][x]+cost );
            vx.push(score);
        }
        vy.push(vx);
    }

    println!("Levenshtein Distance Calc:");
    print_matrix(str1, str2, &vy);
    score
}
