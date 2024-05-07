
class Solution:
    memo={}
    arr=[]
    def obtenerNiveles(self, padre:int, hijo:int) -> int:
        if str(padre)+'|'+str( hijo) not in self.memo:
            maximo=1
            for nieto in self.arr[hijo]:
                if nieto!=padre:
                    maximo=max(self.obtenerNiveles(hijo,nieto)+1,maximo)
            self.memo[str(padre)+'|'+str( hijo)]=maximo
            
        return self.memo[str(padre)+'|'+str( hijo)]
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        self.arr=[[] for node in range(0,n)]
        niveles=[0 for node in range(0,n)]
        
        for edge in edges:
           self.arr[edge[0]].append(edge[1])
           self.arr[edge[1]].append(edge[0])
           lowest=999999
        res=[]
        print(self.arr)
        for i in range(0,n-1):
            for hijo in self.arr[i]:
                niveles=self.obtenerNiveles(i, hijo)
                
                if niveles>lowest:
                    lowest=niveles
                    res=[i]
                if niveles==lowest:
                    res.append(i)
        return res


