impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
       
        let mut prev:String=String::new();
        let mut max:i32=0;
        for letra in s.chars(){
            if let Some(index) =prev.find(letra){
                prev=prev[index+1..].to_string();
            }
            prev=format!("{}{}",prev, letra);
            if max< prev.len() as i32{
                
                max=prev.len() as i32;
            }
        }
        max
    }
}
