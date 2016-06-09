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

#define _GNU_SOURCE /* for hcreate_r() and etc */
#include <search.h>

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <limits.h>
#include <string.h>

#define STR_LEN_INT_MAX (int) (log10(INT_MAX) + 2)

/*
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* twoSum(int* nums, int numsSize, int target) {
    struct hsearch_data htab;
    int i = 0;
    ENTRY e;
    ENTRY *re = NULL;
    char int_str[STR_LEN_INT_MAX];
    int *index_array = NULL;
    int *i_array = NULL;

    i_array = (int *) malloc(sizeof(int) * numsSize);
    if (i_array == NULL)
        return NULL;

    memset(&htab, 0, sizeof(struct hsearch_data));

    if (hcreate_r(numsSize, &htab) == 0) {
        free(i_array);
        return NULL;
    }

    for (; i < numsSize; ++i) {
        i_array[i] = i;
        snprintf(int_str, STR_LEN_INT_MAX, "%d", target - nums[i]);
        e.key = int_str;
        e.data = &i_array[i];
        if (hsearch_r(e, FIND, &re, &htab) != 0) {
            goto found;
        }

        snprintf(int_str, STR_LEN_INT_MAX, "%d", nums[i]);
        e.key = int_str;
        if (hsearch_r(e, ENTER, &re, &htab) == 0)
            goto fail;
    }

 fail:
    free(i_array);
    hdestroy_r(&htab);
    return NULL;

 found:
    index_array = (int *) malloc(sizeof(int) * 2);
    if (index_array == NULL) {
        hdestroy_r(&htab);
        return NULL;
    }

    index_array[0] = * (int *) e.data;
    index_array[1] = * (int *) re->data;
    free(i_array);
    hdestroy_r(&htab);
    return index_array;
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
