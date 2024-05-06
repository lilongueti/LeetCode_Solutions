/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* removeNodes(ListNode* head) {
        bool found =true;
        std::stack<ListNode*> stack;
        ListNode* current=head;
        while(current->next!=nullptr)
        {
            if(current->val<current->next->val)
            {
                if(stack.empty())
                {
                    head=current->next;
                    current=head;
                }
                else
                {
                    current=stack.top();
                    stack.pop();
                    current->next=current->next->next;
                }
            }
            else
            {
                stack.push(current);
                current=current->next;
            }
        }
        return head;
    }
};
