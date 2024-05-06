class boat:
    def __init__(self, free_slot:bool, weight:int, lightest:int):
        self.free_slot=free_slot
        self.weight=weight
        self.lightest=lightest
class Solution:
    
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        boats=[]:[boat]
        while len(people)>0:
            viable_boats=sorted([boat for boat in boats if boat.lightest< people[0] and boat.weight-boat.lightest<=people[0]], key=lambda boat:boat.lightest)


