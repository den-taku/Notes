#include "ArrayStream.h"
#include <iostream>
#include <algorithm>

ArrayStream::ArrayStream(const double *array, int size) {
	std::cout << "ArrayStream" << std::endl;
	m_array = new double[size];
	std::copy(array, array + size, m_array);
	m_i = 0;
}

ArrayStream::~ArrayStream() {
	std::cout << "~ArrayStream" << std::endl;
	delete[] m_array;
}

double ArrayStream::Get() const {
	return m_n;
}

bool ArrayStream::Set() {
	m_n = m_array[m_i];
	if (m_n >= 0) {
		++m_i;
		return true;
	} else {
		return false;
	}
}

void ArrayStream::SetBase() {
	m_n = m_array[m_i];
	if (m_n >= 0) {
		++m_i;
	}
}