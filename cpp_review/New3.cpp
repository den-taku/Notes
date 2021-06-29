#include "ArrayStream.h"
#include <iostream>

int main() {
	static const double ARRAY[] = { 1, 2, 3, -1 };
	static const int SIZE = sizeof ARRAY / sizeof *ARRAY;
	ArrayStream stream(ARRAY, SIZE);
	while(stream.Set()) {
		std::cout << stream.Get() << ' ';
	}
	std::cout << std::endl;
}