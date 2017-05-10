#ifndef UTIL_H_
#define UTIL_H_

#include "rapidjson/rapidjson.h"

bool Eq(const char* s1, rapidjson::SizeType len1, const char* s2, size_t len2);

#endif // UTIL_H_
