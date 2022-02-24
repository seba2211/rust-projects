#include <stdio.h>

int a[5] = {1,2,3,4,5};
char line[1024];

void main() {
    printf("Hello, world! Write a number from 0 to 4\n");

    scanf("%[^\n]", line);

    int x = line[0] - '0';

    printf("arr[%d]=%d\n", x, a[x]);

    // even if a[x] is not defined, C seems to work...
}
