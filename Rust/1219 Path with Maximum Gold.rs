impl Solution {
    pub fn max_gold(grid:&Vec<Vec<i32>>, x:usize,y:usize, rooms:i32)->i32
    {
        
        let mut max=0;
        let row=grid.get(y).unwrap();
        if let Some(cell)=row.get((x-1))
        {
            if *cell>0
            {
                let mut grilla=grid.clone();
                *(grilla.get_mut(y).unwrap().get_mut(x).unwrap())=0;
                max=std::cmp::max(Solution::max_gold(&grilla, x-1,y,rooms+1),max)
            }
        }
        if let Some(cell)=row.get((x+1))
        {
            if *cell>0
            {
                let mut grilla=grid.clone();
                *(grilla.get_mut(y).unwrap().get_mut(x).unwrap())=0;
                max=std::cmp::max(Solution::max_gold(&grilla, x+1,y,rooms+1),max)
            }
        }
        
        if let Some(row)=grid.get((y-1))
        {
            let cell=row.get(x).unwrap();
            if *cell>0
            {
                let mut grilla=grid.clone();
                *(grilla.get_mut(y).unwrap().get_mut(x).unwrap())=0;
                max=std::cmp::max(Solution::max_gold(&grilla, x,y-1,rooms+1),max)
            }
        }
        if let Some(row)=grid.get((y+1))
        {
            let cell=row.get(x).unwrap();
            if *cell>0
            {
                let mut grilla=grid.clone();
                *(grilla.get_mut(y).unwrap().get_mut(x).unwrap())=0;
                max=std::cmp::max(Solution::max_gold(&grilla, x,y+1,rooms+1),max)
            }
        }
        return max+grid.get(y).unwrap().get(x).unwrap();
    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut max=0;
        for y in 0..grid.len()
        {
            let row=grid.get(y).unwrap();
            for x in 0..row.len()
            {
                max=std::cmp::max(Solution::max_gold(&grid, x,y,1),max);
            }
            
        }
        return max;
    }
    
    

}
