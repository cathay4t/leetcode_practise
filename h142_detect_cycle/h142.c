// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode* next;
};

//     a      |     b   |  c      |
//  1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7
//            ^         *         |
//            |                   |
//            +-------------------+
//  define fast finished loop n times and stopped at m location after joint
//  fast: a + (n + 1)b + nc = 2t
//  slow: a + b = t
//  so: a + nb + b + nc = 2a + 2b
//  so: a = bn - b + nc
//  so: a = n(b + c) - b
//  so: a = c + (n-1)(b+c)
//  So when the fast and slow reached, start new ptr walked a times, then joint
//  ptr will equal to this new ptr

struct ListNode* detectCycle(struct ListNode* head) {
    struct ListNode* a = NULL;
    struct ListNode* fast = head;
    struct ListNode* slow = head;
    while (fast != NULL && fast->next != NULL) {
        printf("fast %d\n", fast->val);
        printf("slow %d\n", slow->val);
        fast = fast->next->next;
        slow = slow->next;
        if (fast == slow) {
            a = head;
            break;
        }
    }
    while (a != NULL && slow != NULL) {
        if (slow == a) {
            printf("a %d\n", a->val);
            return a;
        }
        slow = slow->next;
        a = a->next;
    }
    return NULL;
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

    if (detectCycle(head) == node1) {
        printf("PASS\n");
    } else {
        printf("FAIL\n");
    }
}
