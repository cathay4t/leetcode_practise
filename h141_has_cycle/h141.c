// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode* next;
};

bool hasCycle(struct ListNode* head) {
    struct ListNode* fast = head;
    struct ListNode* slow = head;

    while (fast != NULL && fast->next != NULL) {
        if (fast->next == slow) {
            return true;
        }
        printf("fast %d\n", fast->val);
        printf("slow %d\n", slow->val);
        fast = fast->next->next;
        slow = slow->next;
    }
    return false;
}

int main() {
    struct ListNode* node1 = malloc(sizeof(struct ListNode));
    struct ListNode* node2 = malloc(sizeof(struct ListNode));
    struct ListNode* node3 = malloc(sizeof(struct ListNode));
    struct ListNode* head = malloc(sizeof(struct ListNode));
    head->val = 3;
    node1->val = 2;
    node2->val = 0;
    node3->val = -4;
    head->next = node1;
    node1->next = node2;
    node2->next = node3;
    node3->next = node1;

    if (!hasCycle(head)) {
        printf("Failed\n");
    } else {
        printf("PASS\n");
    }
}
