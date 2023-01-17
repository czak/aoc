#include "stdafx.h"

enum class Op {
  inp,
  add, addi,
  mul, muli,
  div, divi,
  mod, modi,
  eql, eqli,
};

using Program = vector<tuple<Op, long, long>>;
using Cpu = array<long, 4>;

Program program{
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 1  },
  // {Op::addi, 1, 12 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 15 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  {Op::inp,  0, 0  },
  {Op::muli, 1, 0  },
  {Op::add,  1, 3  },
  {Op::modi, 1, 26 },
  {Op::divi, 3, 1  },
  {Op::addi, 1, 14 },
  {Op::eql,  1, 0  },
  {Op::eqli, 1, 0  },
  {Op::muli, 2, 0  },
  {Op::addi, 2, 25 },
  {Op::mul,  2, 1  },
  {Op::addi, 2, 1  },
  {Op::mul,  3, 2  },
  {Op::muli, 2, 0  },
  {Op::add,  2, 0  },
  {Op::addi, 2, 12 },
  {Op::mul,  2, 1  },
  {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 1  },
  // {Op::addi, 1, 11 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 15 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, -9 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 12 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, -7 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 15 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 1  },
  // {Op::addi, 1, 11 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 2  },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, -1 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 11 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, -16},
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 15 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 1  },
  // {Op::addi, 1, 11 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 10 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, -15},
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 2  },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 1  },
  // {Op::addi, 1, 10 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 0  },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 1  },
  // {Op::addi, 1, 12 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 0  },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, -4 },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 15 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
  // {Op::inp,  0, 0  },
  // {Op::muli, 1, 0  },
  // {Op::add,  1, 3  },
  // {Op::modi, 1, 26 },
  // {Op::divi, 3, 26 },
  // {Op::addi, 1, 0  },
  // {Op::eql,  1, 0  },
  // {Op::eqli, 1, 0  },
  // {Op::muli, 2, 0  },
  // {Op::addi, 2, 25 },
  // {Op::mul,  2, 1  },
  // {Op::addi, 2, 1  },
  // {Op::mul,  3, 2  },
  // {Op::muli, 2, 0  },
  // {Op::add,  2, 0  },
  // {Op::addi, 2, 15 },
  // {Op::mul,  2, 1  },
  // {Op::add,  3, 2  },
};

Cpu run(Program p, long n, Cpu cpu = {})
{
  deque<long> in{};
  while (n > 0) {
    in.emplace_front(n % 10);
    n /= 10;
  }

  for (const auto& [op, a, b] : p) {
    switch (op) {
      case Op::inp:
        assert(!in.empty());
        cpu[a] = in.front(); in.pop_front();
        break;

      case Op::add:  cpu[a] += cpu[b]; break;
      case Op::addi: cpu[a] += b; break;
      case Op::mul:  cpu[a] *= cpu[b]; break;
      case Op::muli: cpu[a] *= b; break;
      case Op::div:  cpu[a] /= cpu[b]; break;
      case Op::divi: cpu[a] /= b; break;
      case Op::mod:  cpu[a] %= cpu[b]; break;
      case Op::modi: cpu[a] %= b; break;
      case Op::eql:  cpu[a] = (cpu[a] == cpu[b] ? 1 : 0); break;
      case Op::eqli: cpu[a] = (cpu[a] == b ? 1 : 0); break;
    }
  }

  return cpu;
}

int main()
{
  // d1
  // pierwsza cyfra (d1) daje na wyjściu
  // z := d1 + 15
  // czyli po d1, z będzie między 16 a 24

  // d2
  // x := 1
  // z := z * 26 + d2 + 12

  // d3
  // x := 1 zawsze (bo add x 11)
  // z := z * 26 + d3 + 15

  // d4
  // x := 0 jeśli w == ((z % 26) - 9)       // np w = 1 oraz z = 10

  dbg(run(program, 1, {0, 0, 0, 16})[3]);
  dbg(run(program, 1, {0, 0, 0, 17})[3]);
  dbg(run(program, 1, {0, 0, 0, 18})[3]);
  dbg(run(program, 1, {0, 0, 0, 19})[3]);
  dbg(run(program, 1, {0, 0, 0, 20})[3]);
  dbg(run(program, 1, {0, 0, 0, 21})[3]);
  dbg(run(program, 1, {0, 0, 0, 22})[3]);
  dbg(run(program, 1, {0, 0, 0, 23})[3]);
  dbg(run(program, 1, {0, 0, 0, 24})[3]);
}
