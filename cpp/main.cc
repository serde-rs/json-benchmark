// __SSE2__ and __SSE4_2__ are recognized by gcc, clang, and the Intel compiler.
// We use -march=native with gmake to enable -msse2 and -msse4.2, if supported.
#if defined(__SSE4_2__)
#  define RAPIDJSON_SSE42
#elif defined(__SSE2__)
#  define RAPIDJSON_SSE2
#elif defined(_MSC_VER) // Turn on SSE4.2 for VC
#  define RAPIDJSON_SSE42
#endif

#include <fstream>
#include <iomanip>
#include <iostream>

#include "rapidjson/document.h"
#include "rapidjson/error/en.h"
#include "rapidjson/writer.h"

#include "canada.h"
#include "timer.h"

template <typename Handler>
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

  {
    auto min = std::numeric_limits<float>::infinity();
    for (int i = 0; i < 256; i++) {
      rapidjson::Document d;
      timer.reset();
      d.Parse(c_str);
      assert(!d.HasParseError());
      min = std::min(min, timer.millis());
    }
    std::cout << path << " parse dom: " << min << "ms" << std::endl;
  }

  {
    auto min = std::numeric_limits<float>::infinity();
    rapidjson::Document d;
    d.Parse(c_str);
    assert(!d.HasParseError());
    for (int i = 0; i < 256; i++) {
      rapidjson::StringBuffer sb;
      sb.Reserve(s.length());
      rapidjson::Writer<rapidjson::StringBuffer> writer(sb);
      timer.reset();
      auto result = d.Accept(writer);
      assert(result);
      min = std::min(min, timer.millis());
    }
    std::cout << path << " stringify dom: " << min << "ms" << std::endl;
  }

  {
    auto min = std::numeric_limits<float>::infinity();
    for (int i = 0; i < 256; i++) {
      rapidjson::Reader reader;
      Handler handler;
      rapidjson::StringStream ss(c_str);
      timer.reset();
      auto result = reader.Parse(ss, handler);
#if DEBUG
      if (!result) {
        rapidjson::ParseErrorCode err = reader.GetParseErrorCode();
        size_t o = reader.GetErrorOffset();
        std::cerr << "Error: " << rapidjson::GetParseError_En(err) << std::endl;
        std::cerr << s.substr(o, 32) << std::endl;
      }
#endif
      assert(result);
      auto keep = handler.Get();
      min = std::min(min, timer.millis());
    }
    std::cout << path << " parse struct: " << min << "ms" << std::endl;
  }

  {
    auto min = std::numeric_limits<float>::infinity();
    rapidjson::Reader reader;
    Handler handler;
    rapidjson::StringStream ss(c_str);
    timer.reset();
    auto result = reader.Parse(ss, handler);
    assert(result);
    auto d = handler.Get();
    for (int i = 0; i < 256; i++) {
      rapidjson::StringBuffer sb;
      sb.Reserve(s.length());
      rapidjson::Writer<rapidjson::StringBuffer> writer(sb);
      timer.reset();
      Serialize(d, writer);
      min = std::min(min, timer.millis());
    }
    std::cout << path << " stringify struct: " << min << "ms" << std::endl;
  }
}

int main(int argc, char* argv[]) {
  std::cout << std::setiosflags(std::ios::fixed) << std::setprecision(1);

  bench<Canada>("../data/canada.json");
  //bench<CitmCatalog>("../data/citm_catalog.json");
  //bench<Twitter>("../data/twitter.json");
}
