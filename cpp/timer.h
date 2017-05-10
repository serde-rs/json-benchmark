#ifndef TIMER_H_
#define TIMER_H_

#include <chrono>

class Timer {
 public:
  Timer();
  void reset();
  float millis() const;

 private:
  typedef std::chrono::high_resolution_clock clock_;
  typedef std::chrono::duration<float, std::milli> milli_;
  std::chrono::time_point<clock_> begin_;
};

#endif // TIMER_H_
