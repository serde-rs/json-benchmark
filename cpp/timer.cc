#include "timer.h"

Timer::Timer() : begin_(clock_::now()) {}

void Timer::reset() {
  begin_ = clock_::now();
}

float Timer::millis() const {
  return std::chrono::duration_cast<milli_>(clock_::now() - begin_).count();
}
