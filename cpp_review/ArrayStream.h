#ifndef ARRAYSTREAM_H_
#define ARRAYSTREAM_H_

#include "Stream.h"

class ArrayStream :
	public Stream
{
	public:
		ArrayStream(const double* array, int size);
		~ArrayStream();

	public:
		double Get() const;
		bool Set();

	protected:
		virtual void SetBase();

	private:
		double *m_array;
		int m_i;
		double m_n;
};

#endif