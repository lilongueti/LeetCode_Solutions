
class Solution:
    memo={0:0,1:1,2:1}
    def obtenerValor(self, n:int):
        if n not in self.memo:
            self.memo[n]=self.obtenerValor(n-3)+self.obtenerValor(n-2)+self.obtenerValor(n-1)
        return self.memo[n]
    def tribonacci(self, n: int) -> int:
        return self.obtenerValor(n)
