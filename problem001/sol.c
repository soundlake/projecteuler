#include <stdio.h>

int main(int argc, char ** argv) {
    int i = 1, sum = 0;

    for (; i < 1000; i++)
        if (!(i % 3 && i % 5)) sum += i;

    printf("%d\n", sum);
    return 0;
}
