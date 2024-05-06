#include <iostream>

 struct ListNode {
     int val;
     ListNode *next;
     ListNode() : val(0), next(nullptr) {}
     ListNode(int x) : val(x), next(nullptr) {}
     ListNode(int x, ListNode *next) : val(x), next(next) {}
 };

 class Solution {
    public:
        ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
            int res_int_1=l1->val;
            int res_int_2=l2->val;
            while(l1->next!=nullptr)
            {
                l1=l1->next;
                res_int_1=res_int_1*10 + l1->val;
            }
            while(l2->next!=nullptr)
            {
                l2=l2->next;
                res_int_2=res_int_2*10+ l2->val;
            }
            int res_int=res_int_1+res_int_2;
            ListNode* res=new ListNode(res_int);
            /*if(res_int>0)
            {
                std::cout <<"Restante: " << res_int << " modulo 10: "<< res_int%10 << std::endl;
                
                res->val=res_int%10;
                res_int-res->val;
                res_int=res_int/10;
                
            }*/
            ListNode* current=res;
            while(res_int>0)
            {
                std::cout <<"Restante: " << res_int << " modulo 10: "<< res_int%10 << std::endl;
                current->val=res_int%10;
                res_int-current->val;
                res_int=res_int/10;
                std::cout <<"current.val= "<<current->val << " res_int: "<< res_int << std::endl;
                current->next=new ListNode(current->val);
                current=current->next;
            }
            return res;
        }
    };
int main()
{
    Solution sol = Solution();
    sol.addTwoNumbers(new ListNode(2,new ListNode(4, new ListNode(3))), new ListNode(5,new ListNode(6, new ListNode(4))));
    return 0;
    
}
