#define rep(i, n) for (int i = 0; i < (int)(n); ++i )
#include <iostream>
using namespace std;

const int MAX_NAME = 16;

class Student {
	public:
		char name[MAX_NAME + 1];
		int scoreJapanese;
		int scoreMath;
		int scoreEnglish;

		void Show();
};

void Student::Show() {
	cout << "name: " << name << endl
		 << "	Japanese: " << scoreJapanese << " points"
		 << ",	Math: " << scoreMath << " points"
		 << ",	English: " << scoreEnglish << " points" << endl;
}

int main() {
	Student student[] = {
		{ "Red" , 73, 98, 86, },
		{ "Sasa", 64, 45, 40, },
		{ "Old" , 76, 78, 69, },
	};
	int size = sizeof student / sizeof *student;

	rep(i, size) {
		student[i].Show();
	}
}