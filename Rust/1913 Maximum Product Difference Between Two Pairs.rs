impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut n1=0;
        let mut n2=0;
        let mut n3=i32::MAX;
        let mut n4=i32::MAX;
        
        for num in nums
        {
            if n1< num
            {
                n2=n1;
                n1=num;
            }
            else
            {
                n2=std::cmp::max(n2,num);
            }
            if n4>num
            {
                n3=n4;
                n4=num;
            }
            else
            {
                n3=std::cmp::min(n3,num);
            }
            
        }
        
        return (n1*n2) - (n3*n4);
    }
}
