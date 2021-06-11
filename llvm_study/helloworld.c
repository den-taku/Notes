#include <stdio.h>

struct munger_struct {
    int f1;
    int f2;
};

void munge(struct munger_struct *P) {
    P[0].f1 = P[1].f1 + P[2].f2;
}

int branch(int cnd) {
    int i = -1;
    if (cnd == 0) {
        i = 0;
    } else {
        i = 1;
    }
    return i;
}

int main(int argc, char **argv) {
     struct munger_struct Array[3] = {
        {1, 3},
        {9, 7},
        {8, 3},
    };
    munge(Array);
    printf("Hello, world!\n");
    return 0;
}
