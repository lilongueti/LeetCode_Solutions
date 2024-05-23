impl Solution {
    fn test(nums_const:&Vec<i32>, res:&mut i32, k:&i32, current:&mut Vec<i32>){
        let mut nums:Vec<i32>=nums_const.clone();
        
        
        while let Some(next) = nums.pop(){
            
            let mut existe:bool=false;
            for num in current.clone(){
                if (num-next).abs()==*k{
                    existe=true;
                }
            }
            if !existe{
                Solution::test(&nums, res, k, &mut current.clone());
                current.push(next);
                *res=*res+1;
                
            }
        }
    }
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums_mut:Vec<i32>=nums.clone();
        let mut res:i32=nums.len() as i32;
        while let Some(last)=nums_mut.pop(){
            let mut current:Vec<i32>=vec![last];
            Solution::test(&nums_mut, &mut res, &k, &mut current);
        }
        
        return res;
    }
}
