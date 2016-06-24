/*
 * https://leetcode.com/problems/median-of-two-sorted-arrays/
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 * Find the median of the two sorted arrays. The overall run time complexity
 * should be O(log (m+n)).
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <uthash.h>
#include <stdbool.h>

static int VERBOSE = 0;

double medianSingle(int *arr, int n)
{
   if (n == 0)
      return -1;
   if (n % 2 == 0)
        return (arr[n/2] + arr[n/2-1])/2;
   return arr[n/2];
}

double findMedianSortedArrays_o_n(int* nums1, int nums1Size, int* nums2,
                                  int nums2Size)
{
    int i = 0;
    int j = 0;
    int l = 0;
    int m1 = 0;
    int m2 = 0;
    int mv1 = 0;
    int mv2 = 0;

    m1 = nums1Size + nums2Size;

    if (m1 % 2) {
        m1 = (m1 + 1) / 2;
        m2 = m1;
    } else {
        m1 /= 2;
        m2 = m1 + 1;
    }

    while(l != m2) {
        if (i >= nums1Size)
            mv2 = nums2[j++];
        else if (j >= nums2Size)
            mv2 = nums1[i++];
        else {
            if (nums1[i] < nums2[j]) {
                mv2 = nums1[i++];
            } else {
                mv2 = nums2[j++];
            }
        }

        l++;
        if (l == m1)
            mv1 = mv2;
    }

    return ((double) mv1 + (double) mv2) / (double) 2;
}

double _findMedianSortedArrays(int* nums1, int nums1Size, int* nums2,
                               int nums2Size)
{
    int idx1 = (nums1Size - 1) / 2;
    int idx2 = (nums2Size - 1) / 2;

    if (VERBOSE == 1) {
        printf("nums1: %d\n", nums1[0]);
        printf("nums1Size: %d\n", nums1Size);
        printf("nums2: %d\n", nums2[0]);
        printf("nums2Size: %d\n", nums2Size);
    }

    if ((nums1Size <= 2) || (nums2Size <= 2))
        return findMedianSortedArrays_o_n(nums1, nums1Size,
                                          nums2, nums2Size);

    if (VERBOSE == 1) {
        printf("idx1 number: %d\n", nums1[idx1]);
        printf("idx2 number: %d\n", nums2[idx2]);
    }

    if (nums1[idx1] <= nums2[idx2])
        return _findMedianSortedArrays(nums1 + idx1,
                                       nums1Size/2 + 1,
                                       nums2,
                                       nums2Size - idx1);
    else
        return _findMedianSortedArrays(nums1,
                                       nums1Size/2 + 1,
                                       nums2 + idx1,
                                       nums2Size - idx1);
}

double findMedianSortedArrays(int* nums1, int nums1Size, int* nums2,
                              int nums2Size)
{
    if (nums1Size > nums2Size)
        return _findMedianSortedArrays(nums2, nums2Size, nums1, nums1Size);
    return _findMedianSortedArrays(nums1, nums1Size, nums2, nums2Size);
}

int main(int argc, char **argv) {
    struct data {
        int*    n1;
        int     s1;
        int*    n2;
        int     s2;
        double  re;
    };

    int rc = EXIT_SUCCESS;
    int i = 0;
    int j = 0;
    double tmp = 0;
    struct data a[] = {
        {
            (int []) {},                            0,
            (int []) {1},                           1,
            1
        },
        {
            (int []) {1},                           1,
            (int []) {1},                           1,
            1
        },
        {
            (int []) {1},                           1,
            (int []) {9},                           1,
            5
        },
        {
            (int []) {9},                           1,
            (int []) {1,3},                         2,
            3
        },
        {
            (int []) {1,2},                         2,
            (int []) {1,2},                         2,
            1.5},
        {
            (int []) {1,2},                         2,
            (int []) {1,1},                         2,
            1
        },
        {
            (int []) {1,5},                         2,
            (int []) {2,6},                         2,
            3.5
        },
        {
            (int []) {1,2,3},                       3,
            (int []) {4,5},                         2,
            3
        },
        {
            (int []) {1,2},                         2,
            (int []) {1,2,3},                       3,
            2
        },
        {
            (int []) {1},                           1,
            (int []) {2,3,4},                       3,
            2.5
        },
        {
            (int []) {2},                           1,
            (int []) {1,3,4},                       3,
            2.5
        },
        {
            (int []) {2,7,10},                      3,
            (int []) {1,3,4},                       3,
            3.5
        },
        {
            (int []) {1,2,3},                      3,
            (int []) {4,5,6,7},                    4,
            4
        },
        {
            (int []) {1,2,3},                      3,
            (int []) {4,5,6,7,8},                  5,
            4.5
        },
        {
            (int []) {1,5,6},                      3,
            (int []) {2,3,4,7,8},                  5,
            4.5
        },
    };

    if (argc > 1)
        VERBOSE = 1;

    for (; i < sizeof(a)/sizeof(struct data); ++i) {
        tmp = findMedianSortedArrays(a[i].n1, a[i].s1,
                                     a[i].n2, a[i].s2);
        if (tmp == a[i].re)
            printf("PASS: got %f\n", tmp);
        else {
            printf("FAIL: expect %f vs got %f\n", a[i].re, tmp);
            rc = EXIT_FAILURE;
        }

        printf("\t{");
        for (j = 0; j < a[i].s1; ++j)
            printf("%d, ", a[i].n1[j]);
        printf("}\n");

        printf("\t{");
        for (j = 0; j < a[i].s2; ++j)
            printf("%d, ", a[i].n2[j]);
        printf("}\n");
    }

    exit(rc);
}
