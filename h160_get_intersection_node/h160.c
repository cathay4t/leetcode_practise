// SPDX-License-Identifier: Apache-2.0

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode* next;
};

struct ListNode* add_node(struct ListNode* tail, int val) {
    tail->next = malloc(sizeof(struct ListNode));
    tail->next->val = val;
    return tail->next;
}

size_t getListLength(struct ListNode* head) {
    size_t ret = 0;
    while (head != NULL) {
        ret += 1;
        head = head->next;
    }
    return ret;
}

struct ListNode* forward(struct ListNode* head, size_t count) {
    for (size_t i = 0; i < count && head != NULL; ++i) {
        head = head->next;
    }
    return head;
}

struct ListNode* getIntersectionNode(struct ListNode* head_a,
                                     struct ListNode* head_b) {
    // first make two list holding the same length
    size_t size_a = getListLength(head_a);
    size_t size_b = getListLength(head_b);

    if (size_a > size_b) {
        head_a = forward(head_a, size_a - size_b);
    } else if (size_a < size_b) {
        head_b = forward(head_b, size_b - size_a);
    }

    // check one by one
    while (head_a != NULL && head_b != NULL) {
        if (head_a == head_b) {
            return head_a;
        }
        head_a = head_a->next;
        head_b = head_b->next;
    }
    return NULL;
}

int main() {
    struct ListNode* tail_a = NULL;
    struct ListNode* tail_b = NULL;
    struct ListNode* tail_j = NULL;
    struct ListNode* list_a = malloc(sizeof(struct ListNode));
    struct ListNode* list_b = malloc(sizeof(struct ListNode));
    struct ListNode* list_join = malloc(sizeof(struct ListNode));

    list_a->val = 4;
    tail_a = add_node(list_a, 1);

    list_b->val = 5;
    tail_b = add_node(list_b, 6);

    list_join->val = 8;
    tail_j = list_join;
    tail_j = add_node(tail_j, 4);
    tail_j = add_node(tail_j, 5);

    tail_a->next = list_join;
    tail_b->next = list_join;

    if (getIntersectionNode(list_a, list_b) == list_join) {
        printf("PASS\n");
        return EXIT_SUCCESS;
    } else {
        printf("FAIL\n");
        return EXIT_FAILURE;
    }
}
