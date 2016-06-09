/*
 * https://leetcode.com/problems/add-two-numbers/
 *
 * You are given two linked lists representing two non-negative numbers. The
 * digits are stored in reverse order and each of their nodes contain a single
 * digit. Add the two numbers and return it as a linked list.
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    uint_fast8_t carry = 0;
    struct ListNode *cur_l1 = l1;
    struct ListNode *cur_l2 = l2;
    struct ListNode *out = NULL;
    struct ListNode *cur_out = NULL;

    while((cur_l1 != NULL) || (cur_l2 != NULL)) {
        if (out == NULL) {
            out = (struct ListNode *) malloc(sizeof(struct ListNode));
            if (out == NULL)
                return NULL;

            cur_out = out;
        } else {
            cur_out->next = (struct ListNode *) malloc(sizeof(struct ListNode));
            if (cur_out->next == NULL)
                return NULL;
            cur_out = cur_out->next;
        }
        if (cur_l1 != NULL) {
            carry += cur_l1->val;
            cur_l1 = cur_l1->next;
        }
        if (cur_l2 != NULL) {
            carry += cur_l2->val;
            cur_l2 = cur_l2->next;
        }

        if (carry >= 10) {
            cur_out->val = carry - 10;
            carry = 1;
        } else {
            cur_out->val = carry;
            carry = 0;
        }
        cur_out->next = NULL;
    }
    if (carry != 0) {
        cur_out->next = (struct ListNode *) malloc(sizeof(struct ListNode));
        if (cur_out->next == NULL)
            return NULL;
        cur_out = cur_out->next;
        cur_out->val = 1;
        cur_out->next = NULL;
    }

    return out;
}

int main(void) {
    struct ListNode *l1 = NULL;
    struct ListNode *l2 = NULL;
    struct ListNode l1_data[3];
    struct ListNode l2_data[3];
    struct ListNode *out = NULL;
    struct ListNode *tmp = NULL;

    l1_data[0].val = 2;
    l1_data[0].next = &l1_data[1];

    l1_data[1].val = 4;
    l1_data[1].next = &l1_data[2];

    l1_data[2].val = 3;
    l1_data[2].next = NULL;

    l1 = l1_data;

    l2_data[0].val = 5;
    l2_data[0].next = &l2_data[1];

    l2_data[1].val = 6;
    l2_data[1].next = &l2_data[2];

    l2_data[2].val = 4;
    l2_data[2].next = NULL;

    l2 = l2_data;

    out = addTwoNumbers(l1, l2);
    if (out == NULL) {
        printf("Got NULL return from addTwoNumbers()\n");
        exit(EXIT_FAILURE);
    }
    printf("Results:\n");
    while(out != NULL) {
        printf("%d\n", out->val);
        tmp = out->next;
        free(out);
        out = tmp;
    }
    exit(EXIT_SUCCESS);
}
