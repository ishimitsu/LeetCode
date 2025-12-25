#include <iostream>
#include <vector>

using std::vector;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode *nodePtr = head;
        ListNode *removeTarget = head;
        ListNode *prevRemove = head;
        int nodeIdx = 0;

        while (nodePtr != nullptr) {
            nodePtr = nodePtr->next;
            nodeIdx++;
            if (nodeIdx > n && removeTarget->next != nullptr) {
                prevRemove = removeTarget;
                removeTarget = removeTarget->next;
            }
        }

        if (removeTarget == head) {
            // removeTarget doesn't move, and n request removing head
            return head = head->next;
        } else {
            prevRemove->next = removeTarget->next;
        }

        return head;
    }
};

#include <gtest/gtest.h>

class RemoveNthNodeFromEndOfListTest : public ::testing::Test {
protected:
    Solution solution;

    // Helper function to create linked list from vector
    ListNode* createList(const vector<int>& vals) {
        if (vals.empty()) return nullptr;
        ListNode* head = new ListNode(vals[0]);
        ListNode* current = head;
        for (size_t i = 1; i < vals.size(); i++) {
            current->next = new ListNode(vals[i]);
            current = current->next;
        }
        return head;
    }

    // Helper function to convert linked list to vector
    vector<int> listToVector(ListNode* head) {
        vector<int> result;
        while (head != nullptr) {
            result.push_back(head->val);
            head = head->next;
        }
        return result;
    }

    // Helper function to delete linked list
    void deleteList(ListNode* head) {
        while (head != nullptr) {
            ListNode* temp = head;
            head = head->next;
            delete temp;
        }
    }
};

// Example 1: head = [1,2,3,4,5], n = 2 -> [1,2,3,5]
TEST_F(RemoveNthNodeFromEndOfListTest, Example1) {
    ListNode* head = createList({1, 2, 3, 4, 5});
    ListNode* result = solution.removeNthFromEnd(head, 2);
    EXPECT_EQ(listToVector(result), vector<int>({1, 2, 3, 5}));
    deleteList(result);
}

// Example 2: head = [1], n = 1 -> []
TEST_F(RemoveNthNodeFromEndOfListTest, Example2) {
    ListNode* head = createList({1});
    ListNode* result = solution.removeNthFromEnd(head, 1);
    EXPECT_EQ(listToVector(result), vector<int>({}));
    deleteList(result);
}

// Example 3: head = [1,2], n = 1 -> [1]
TEST_F(RemoveNthNodeFromEndOfListTest, Example3) {
    ListNode* head = createList({1, 2});
    ListNode* result = solution.removeNthFromEnd(head, 1);
    EXPECT_EQ(listToVector(result), vector<int>({1}));
    deleteList(result);
}

// Example 4: head = [1,2], n = 2 -> [2]
TEST_F(RemoveNthNodeFromEndOfListTest, Example4) {
    ListNode* head = createList({1, 2});
    ListNode* result = solution.removeNthFromEnd(head, 2);
    EXPECT_EQ(listToVector(result), vector<int>({2}));
    deleteList(result);
}
int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
