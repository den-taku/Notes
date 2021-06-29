#include <iostream>
using namespace std;

enum CompareResult {
	CR_LESS_THAN = 0,
	CR_EQUAL_TO = 1,
	CR_GREATER_THAN = 2,
};

CompareResult Compare(int a, int b) {
	if (a < b) {
		return CR_LESS_THAN;
	} else if (a > b) {
		return CR_GREATER_THAN;
	} else {
		return CR_LESS_THAN;
	}
}

bool Compare() {
	int a, b;

	cout << "Input 2 numbers > " << flush;
	cin >> a >> b;

	if (a == -1) {
		return false;
	}

	switch (Compare(a, b)) {
		case CR_LESS_THAN : 
			cout << "prior is less than latter." << endl;
			break;

		case CR_EQUAL_TO : 
			cout << "both is equal." << endl;
			break;
		
		case CR_GREATER_THAN : 
			cout << "prior is greater than latter." << endl;
			break;
	}

	return true;
}

int main() {
	while(Compare()) {}
	return 0;
}
