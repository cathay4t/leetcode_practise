// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Node {
    int val;
    struct Node* next;
    struct Node* random;
};

// return the new node
struct Node* add_node(struct Node* node, int val) {
    struct Node* new_node = calloc(1, sizeof(struct Node));
    // TODO: Handle NULL return of calloc
    new_node->val = val;
    node->next = new_node;
    return new_node;
}

void print_list(struct Node* node) {
    while (node != NULL) {
        if (node->random != NULL) {
            printf("val %d, random %d\n", node->val, node->random->val);
        } else {
            printf("val %d, random NULL\n", node->val);
        }
        node = node->next;
    }
}

struct Node* copyRandomList(struct Node* head) {
    struct Node* old_next = NULL;
    struct Node* old_node = head;
    struct Node* new_node = NULL;
    struct Node* new_head = NULL;


    if (head == NULL) {
        return NULL;
    }
    // Define A' as clone of A
    // For List A->B->C, we create A->A'->B->B'->C->C'
    while (old_node != NULL) {
        old_next = old_node->next;
        new_node = malloc(sizeof(struct Node));
        //TODO: Handle NULL
        memcpy(new_node, old_node, sizeof(struct Node));
        new_node->next = old_next;
        old_node->next = new_node;
        old_node = old_next;
    }
    new_head = head->next;
    new_node = new_head;
    // Update new list's random
    while (new_node != NULL) {
        if (new_node->random != NULL) {
            new_node->random = new_node->random->next;
        }
        if (new_node->next != NULL) {
            new_node = new_node->next->next;
        } else {
            break;
        }
    }
    // Split this long list into two
    new_node = new_head;
    old_node = head;
    while (old_node != NULL && new_node != NULL) {
        old_node->next = new_node->next;
        if (new_node->next != NULL) {
            new_node->next = new_node->next->next;
        }
        old_node = old_node->next;
        new_node = new_node->next;
    }
    return new_head;
}

int main() {
    struct Node* head = calloc(1, sizeof(struct Node));
    // TODO: handle NULL return of malloc
    struct Node* tail = head;
    struct Node** ptr_array = calloc(5, sizeof(struct Node*));
    // TODO: handle NULL return of calloc
    ptr_array[0] = head;

    head->val = 7;

    tail = add_node(tail, 13);
    ptr_array[1] = tail;
    tail = add_node(tail, 11);
    ptr_array[2] = tail;
    tail = add_node(tail, 10);
    ptr_array[3] = tail;
    tail = add_node(tail, 1);
    ptr_array[4] = tail;

    ptr_array[0]->random = NULL;
    ptr_array[1]->random = ptr_array[0];
    ptr_array[2]->random = ptr_array[4];
    ptr_array[3]->random = ptr_array[2];
    ptr_array[4]->random = ptr_array[0];

    print_list(head);

    printf("\nNEW LIST\n");
    struct Node* new_list = copyRandomList(head);

    print_list(head);
    return EXIT_SUCCESS;
}
