import time
class Solution:
    
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        startTime=time.time()
        people.sort()
        print("tiempo sort="+str(startTime-time.time()))
        res=0
        primero=0
        ultimo=len(people)-1
        while ultimo>primero:
            if people[primero] + people[ultimo]>limit:
                ultimo-=1
                res+=1
            else:
                primero=primero+1
                ultimo-=1
                res+=1
        if primero==ultimo:
            res+=1
        print("tiempo terminar="+str(startTime-time.time()))
        return res