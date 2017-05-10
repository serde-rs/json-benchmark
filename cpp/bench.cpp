// __SSE2__ and __SSE4_2__ are recognized by gcc, clang, and the Intel compiler.
// We use -march=native with gmake to enable -msse2 and -msse4.2, if supported.
#if defined(__SSE4_2__)
#  define RAPIDJSON_SSE42      
#elif defined(__SSE2__)        
#  define RAPIDJSON_SSE2
#elif defined(_MSC_VER) // Turn on SSE4.2 for VC
#  define RAPIDJSON_SSE42
#endif

#include <chrono>
#include <fstream>
#include <iomanip>
#include <iostream>

#include "rapidjson/document.h"
#include "rapidjson/writer.h"

class Timer {
 public:
  Timer() : begin_(clock_::now()) {}
  void reset() {
    begin_ = clock_::now();
  }
  float millis() const { 
    return std::chrono::duration_cast<milli_>(clock_::now() - begin_).count();
  }

 private:
  typedef std::chrono::high_resolution_clock clock_;
  typedef std::chrono::duration<float, std::milli> milli_;
  std::chrono::time_point<clock_> begin_;
};

void bench(const char* path) {
  std::ifstream in(path, std::ios::in | std::ios::binary);
  std::string s;
  in.seekg(0, std::ios::end);
  s.resize(in.tellg());
  in.seekg(0, std::ios::beg);
  in.read(&s[0], s.size());
  in.close();
  auto c_str = s.c_str();

  Timer timer;

  auto min_parse = std::numeric_limits<float>::infinity();
  for (int i = 0; i < 256; i++) {
    rapidjson::Document d;
    timer.reset();
    d.Parse(c_str);
    min_parse = std::min(min_parse, timer.millis());
  }
  std::cout << path << " parse dom: " << min_parse << "ms" << std::endl;

  auto min_stringify = std::numeric_limits<float>::infinity();
  rapidjson::Document d;
  d.Parse(c_str);
  for (int i = 0; i < 256; i++) {
    rapidjson::StringBuffer sb;
    sb.Reserve(s.length());
    rapidjson::Writer<rapidjson::StringBuffer> writer(sb);
    timer.reset();
    d.Accept(writer);
    min_stringify = std::min(min_stringify, timer.millis());
  }
  std::cout << path << " stringify dom: " << min_stringify << "ms" << std::endl;
}

int main(int argc, char* argv[]) {
  std::cout << std::setiosflags(std::ios::fixed) << std::setprecision(1);

  bench("../data/canada.json");
  bench("../data/citm_catalog.json");
  bench("../data/twitter.json");
}
