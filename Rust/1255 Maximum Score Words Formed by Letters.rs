impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        Solution::solucion(&words, &letters, &score)
    }
    fn solucion(words: &Vec<String>, letters: &Vec<char>, score: &Vec<i32>)->i32{
        let mut letters=letters.clone();
        let mut total_score=0;
        let mut words=words.clone();
        let mut skipped=0;
        while let Some(word) = words.pop(){
            skipped=skipped.max(Solution::solucion(&words, &letters, score)+total_score);
            let mut w_score=0;
            {
                let mut inner_letters=letters.clone();
                for letra in word.chars(){
                    match inner_letters.iter().position(|&x| x==letra){
                        Some(indx)=>{
                            inner_letters.remove(indx);
                            
                            }
                        None=>{
                            w_score=0;
                            inner_letters=letters;
                            break;
                        }
                    }
                    w_score=w_score+score[letra as usize -97];
                    
                }
                letters=inner_letters;
                
               
            }
            total_score=total_score+ w_score;
        }
        total_score.max(skipped)
    }
}
