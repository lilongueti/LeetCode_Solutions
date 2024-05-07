import time
class Solution:
    def finalString(self, s: str) -> str:
        reversed=False
        res=''
        startTime=time.time()

        for letra in s:
            if letra=='i':
                reversed= reversed!=True
            else:
                if reversed==False:
                    res=res+letra
                else:
                    res=letra+res
            
        #print("t1 "+str(time.time()-startTime))
        if reversed:
            startTime=time.time()
            res=res[::-1]
            #print("t2 "+str(time.time()-startTime))
        return res
        
        
