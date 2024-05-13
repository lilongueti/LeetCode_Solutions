impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res:i64=0;
        let mut input:i64=i64::from(x);
        let negativo:bool;
        if(x<0)
        {
            negativo=true;
            input=i64::from(-1*x);
        }
        else 
        {
            negativo=false;
        }
        while(input>0)
        {
            res=res*10;
            res=res+input%10;
            input=(input-input%10)/10;
        }
        if(negativo)
        {
            res=res*-1;
        }
        
        let conversion=i32::try_from(res);
        match conversion
        {
            Ok(val)=>{
                return val;
            }
            Err(_)=>{
                return 0;
            }
        }
    }
}
