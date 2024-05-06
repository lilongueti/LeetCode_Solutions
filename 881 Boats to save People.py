
import time
class Solution:
    
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        startTime=time.time()
        people.sort()
        print("tiempo sort="+str(startTime-time.time()))
        print("termina sort")
        res=0
        while len(people)>0:
            if len(people)==1:
                res=res+1
                print("tiempo terminar="+str(startTime-time.time()))
                return res
            if people[0] + people[len(people)-1]>limit:
                people.remove(people[len(people)-1])
                res=res+1
            else:
                people.remove(people[0])
                people.remove(people[len(people)-1])
                res=res+1
        print("tiempo terminar="+str(startTime-time.time()))
        return res
