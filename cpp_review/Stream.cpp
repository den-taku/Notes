#include "Stream.h"
#include <iostream>

Stream::Stream() : m_n(-1) {
	std::cout << "Stream" << std::endl;
	// m_n = -1;
}

Stream::Stream(double n) {
	std::cout << "Stream" << std::endl;
	m_n = n;
}

Stream::~Stream() {
	std::cout << "~Steam" << std::endl;
}

double Stream::Get() const {
	return m_n;
}

bool Stream::Set() {
	std::cout << "Stream::Set" << std::endl;
	m_n = -1;
	return false;
}

// void Stream::SetBase() {
// 	std::cout << "Stream::SetBase" << std::endl;
// 	m_n = -1;
// }