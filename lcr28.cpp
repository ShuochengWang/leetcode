/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* prev;
    Node* next;
    Node* child;
};
*/

class Solution {
public:
    Node* flatten(Node* head) {
        dfs(head);
        return head;
    }
private:
    Node* dfs(Node* head) {
        Node* cur = head;
        Node* last = NULL;
        while (cur) {
            Node* next = cur->next;
            if (cur->child) {
                Node* child_last = dfs(cur->child);
                cur->next = cur->child;
                cur->next->prev = cur;
                if (next) {
                    child_last->next = next;
                    next->prev = child_last;
                }
                cur->child = NULL;
                last = child_last;
            } else {
                last = cur;
            }
            cur = next;
        }
        return last;
    }
};