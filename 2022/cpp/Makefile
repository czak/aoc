CXXFLAGS = -std=c++17 -Iinclude -Wall

day11: day11.cpp day11_regex.o include/utils.h
	$(CXX) $(CXXFLAGS) day11.cpp day11_regex.o -o $@

day15: CXXFLAGS += -O2

.PHONY: precompile
precompile:
	$(CXX) $(CXXFLAGS) include/stdafx.h
