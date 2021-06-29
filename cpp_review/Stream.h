#ifndef STREAM_H_
#define STREAM_H_

class Stream {
	public:
		Stream();
		Stream(double n);
		virtual ~Stream();
	public:
		double Get() const;
		bool Set();
	protected:
		virtual void SetBase() = 0;
	protected:
		double m_n;
};

#endif