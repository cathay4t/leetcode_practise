/*
 * https://leetcode.com/problems/two-sum/
 * Given an array of integers, find two numbers such that they add up to a
 * specific target number.
 *
 * The function twoSum should return indices of the two numbers such that they
 * add up to the target, where index1 must be less than index2. Please note that
 * your returned answers (both index1 and index2) are not zero-based.
 *
 * You may assume that each input would have exactly one solution.
 *
 * Input: numbers={2, 7, 11, 15}, target=9
 * Output: index1=0, index2=1
 */

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>
#include <uthash.h>

/*
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* twoSum(int* nums, int numsSize, int target) {
    struct my_struct {
        int key;
        int value;
        UT_hash_handle hh;
    };

    int i = 0;
    struct my_struct *hash_table = NULL;
    struct my_struct data[numsSize];
    int tmp_int = 0;
    struct my_struct *search_result = NULL;
    int *index_array = NULL;

    for (; i < numsSize; ++i) {
        tmp_int = target - nums[i];
        if (hash_table != NULL)
            HASH_FIND_INT(hash_table, &tmp_int, search_result);

        if (search_result != NULL) {
            index_array = (int *) malloc(sizeof(int) * 2);
            if (index_array == NULL)
                return NULL;
            if (search_result->value > i) {
                index_array[0] = i;
                index_array[1] = search_result->value;
            } else {
                index_array[0] = search_result->value;
                index_array[1] = i;
            }
            HASH_CLEAR(hh, hash_table);
            return index_array;
        }

        data[i].key = nums[i];
        data[i].value = i;
        HASH_ADD_INT(hash_table, key, &data[i]);
    }

    return NULL;
}

int main(void) {
    int *index_array = NULL;
    int input_nums[] = {2, 7, 11, 15, 20, 32, 45, 90, 100, 102, 109, 584, 700};
    int target = 702;

    index_array = twoSum(input_nums, sizeof(input_nums)/sizeof(input_nums[0]),
                         target);
    if (index_array == NULL) {
        printf("No match\n");
        exit(EXIT_FAILURE);
    }

    printf("index1=%d\n", index_array[0]);
    printf("index2=%d\n", index_array[1]);

    free(index_array);
    exit(EXIT_SUCCESS);
}
