# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def doubleIt(self, head: Optional[ListNode]) -> Optional[ListNode]:
        stack=[]
        current=head
        while current !=None:
            stack.append(current)
            current=current.next
        carryFlag=0
        while len(stack)>0:
            current=stack.pop()
            current.val=current.val*2+carryFlag
            if current.val>=10:
                current.val-=10
                carryFlag=1
            else:
                carryFlag=0
        if carryFlag==1:
            head=ListNode(carryFlag,current)
        return head
