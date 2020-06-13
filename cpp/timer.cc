#include "timer.h"

Timer::Timer() : begin_(clock_::now()) {}

void Timer::reset() { begin_ = clock_::now(); }

std::chrono::microseconds Timer::micros() const {
  return std::chrono::duration_cast<std::chrono::microseconds>(clock_::now() -
                                                               begin_);
}

// matches json-benchmark/src/lib.rs
uint64_t throughput(std::chrono::microseconds dur, size_t bytes) {
  auto mbps = bytes / static_cast<uint64_t>(dur.count());

  // Round to two significant digits.
  if (mbps > 100) {
    if (mbps % 10 >= 5) {
      mbps += 10;
    }
    mbps = mbps / 10 * 10;
  }

  return mbps;
}
