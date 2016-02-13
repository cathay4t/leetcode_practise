/*
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
 * Output: index1=1, index2=2
 */

#include <stdio.h>
#include <stdlib.h>

/*
 * Note: The returned array must be malloced, assume caller calls free().
 *
 */
int* twoSum(int* nums, int numsSize, int target) {
    int *index_array = NULL;

    index_array = (int *) malloc(sizeof(int) * 2);
    if (index_array == NULL)
        return NULL;

    for (index_array[0] = 0; index_array[0] < numsSize; ++index_array[0]) {
        for (index_array[1] = 0; index_array[1] < numsSize; ++index_array[1]) {
            if (index_array[0] == index_array[1])
                continue;
            if ((nums[index_array[0]] + nums[index_array[1]]) == target)
                goto found;
        }
    }

    free(index_array);
    return NULL;
 found:
    ++index_array[0];
    ++index_array[1];
    return index_array;
}

int main(void) {
    int *index_array = NULL;
    int input_nums[] = {2, 7, 11, 15};
    int target = 9;

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
