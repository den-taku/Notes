#include <iostream>
#include <string>
#include <tuple>
#include <time.h>

enum Lab {
    Sato,
    Hashimoto,
    Nakamura,
    Doi,
    Kurohashi,
    Nishino,
    Morikura
};

using tup = std::tuple<bool, Lab>;

inline void InitRand() {
    srand((unsigned int)time(NULL));
}

inline int Dice() {
    return rand() % 7 + 1;
}

inline bool Decide(tup array[], int choice) {
    if (! std::get<0>(array[choice])) {
        std::get<0>(array[choice]) = true;
        return true;
    } else {
        return false;
    }
}

char* enum_to_string(Lab e) {
    switch(e) {
        case Sato:
            return "Sato";
        case Hashimoto:
            return "Hashimoto";
        case Nakamura:
            return "Nakamura";
        case Doi:
            return "Doi";
        case Kurohashi:
            return "Kurohashi";
        case Nishino:
            return "Nishino";
        case Morikura:
            return "Morikura";
    }
}

int main() {
    int i;
    InitRand();

    tup choices[] = {
        std::make_tuple(false, Sato),
        std::make_tuple(false, Hashimoto),
        std::make_tuple(false, Nakamura),
        std::make_tuple(false, Doi),
        std::make_tuple(false, Kurohashi),
        std::make_tuple(false, Nishino),
        std::make_tuple(false, Morikura),
    };

    Dice();

    for (i = 1; i < 8; ) {
        int choice = Dice();
        if (Decide(choices, choice)) {
            std::cout << i << ": " << enum_to_string(static_cast<Lab>(choice)) << std::endl;
            ++i;
        }
    }

    return 0;
}
