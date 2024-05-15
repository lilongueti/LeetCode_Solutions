impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid=grid.clone();
        for row in grid.iter_mut()
        {
            if row[0]==0
            {
                for cell in row.iter_mut()
                {
                    if *cell==1
                    {
                        *cell=0;
                    }
                    else
                    {
                        *cell=1;
                    }
                }
            }
        }
        
        for i in 1..grid[0].len()
        {
            let mut acum1=0;
            let mut acum0=0;
            for row in grid.iter()
            {
                if row[i]==1
                {
                    acum1+=1;
                }
                else
                {
                    acum0+=1;
                }
            }
            
            if acum0>acum1
            {
                for row in grid.iter_mut()
                {
                    if row[i]==1
                    {
                        row[i]=0;
                    }
                    else
                    {
                        row[i]=1
                    }
                }
            }
        }
        let mut res=0;
        for row in grid.iter()
        {
            let mut val=0;
            let mut exp=1;
            for i in (0..row.len()).rev()
            {
                val=val+exp*row[i];
                exp=exp*2;
                
            }
            res=res+val;
            
        }
        return res;
    }
}
