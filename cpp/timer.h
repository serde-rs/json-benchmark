#ifndef TIMER_H_
#define TIMER_H_

#include <chrono>

class Timer {
 public:
  Timer();
  void reset();
  std::chrono::microseconds micros() const;

 private:
  typedef std::chrono::high_resolution_clock clock_;
  std::chrono::time_point<clock_> begin_;
};

uint64_t throughput(std::chrono::microseconds dur, size_t bytes);

#endif // TIMER_H_
