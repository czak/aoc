#include "stdafx.h"

using namespace std;

template<typename Container>
Container parse_to(const string& s, const regex& r, function<typename Container::value_type(const smatch&)> fn)
{
  Container items{};
  transform(sregex_iterator{s.begin(), s.end(), r}, {}, back_inserter(items), fn);
  return items;
}
