#include <stdio.h>

#define pow(a)((a)*(a))

int main(void){
    int a=2;

    // 6, 20
    printf("%d, ",pow(a++));
    printf("%d\n",pow(a++));
}
