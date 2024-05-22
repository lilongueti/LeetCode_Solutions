class Solution:
    res=[]
    def subsets(self, nums: List[int]) -> List[List[int]]:
        
        self.res=[[]]
        self.calc(nums,[])
        

        return self.res
    def calc(self, nums:List[int], current:List[int]):
        for i in range(0,len(nums)):
            l=[num for num in current]+[nums[i]]
            self.res.append(l)
            
            self.calc(nums[i+1:len(nums)],l)

