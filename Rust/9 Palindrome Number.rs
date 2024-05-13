impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if(x<0)
        {
            return false;
        }
        let mut res=0;
        let mut input=x;
        while(input>0)
        {
            res=res*10;
            res=res+input%10;
            input=input-input%10;
            input=input/10;
        }
        if(res==x)
        {
            return true;
        }
        return false;
    }
}
