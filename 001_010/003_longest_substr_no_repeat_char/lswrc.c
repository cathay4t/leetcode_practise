/*
 * https://leetcode.com/problems/longest-substring-without-repeating-characters/
 *
 * Given a string, find the length of the longest substring without repeating
 * characters.
 *
 * Examples:
 *
 * Given "abcabcbb", the answer is "abc", which the length is 3.
 *
 * Given "bbbbb", the answer is "b", with the length of 1.
 *
 * Given "pwwkew", the answer is "wke", with the length of 3. Note that the
 * answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <uthash.h>
#include <limits.h>     /* for CHAR_BIT */
#include <stdbool.h>

int lengthOfLongestSubstring(char* s) {
    int max_len = 1;
    size_t i = 0;
    size_t j = 0;
    int *bitmap = NULL;
    ssize_t bitmap_len = (1 << CHAR_BIT ) * sizeof(int);
    ssize_t len = strlen(s);

    if (len == 0)
        return 0;

    bitmap = (int *) malloc(bitmap_len);
    memset(bitmap, 0, bitmap_len);

    for (; j < len; ++j) {
        i = *(bitmap + s[j]) > i ? *(bitmap + s[j]) : i;
        *(bitmap + s[j]) = j + 1;
        max_len = max_len > j - i + 1 ? max_len : j - i + 1;
    }

    free(bitmap);
    return max_len;
}

int main(void) {
    printf("Results: %d\n", lengthOfLongestSubstring("abcabcbb"));
    printf("Results: %d\n", lengthOfLongestSubstring("pwwkew"));
    printf("Results: %d\n", lengthOfLongestSubstring("1234514321"));
    printf("Results: %d\n", lengthOfLongestSubstring("ddvdf"));
    printf("Results: %d\n", lengthOfLongestSubstring("tmmzuxt"));
    exit(EXIT_SUCCESS);
}
