#include "util.h"

bool Eq(const char* s1, rapidjson::SizeType len1, const char* s2, size_t len2) {
  return len1 == len2 && strncmp(s1, s2, len1) == 0;
}
