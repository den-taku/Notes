#include <iostream>
#include <time.h>

inline void InitRand() {
    srand((unsigned int)time(NULL));
}

inline int Dice() {
    return rand() % 7 + 1;
}

int main() {
    int i;
    InitRand();

    for (i = 0; i < 20; ++i) {
        std::cout << Dice() << ' ';
    }
    std::cout << std::flush;

    return 0;
}
