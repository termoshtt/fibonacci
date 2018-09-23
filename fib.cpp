
#include <cstdint>

constexpr std::size_t MAX_N = 1000;

std::size_t memo[MAX_N + 1];

extern "C" std::size_t fib_cpp(std::size_t n) {
    if (n <= 1) return n;
    if (memo[n] != 0) return memo[n];
    return memo[n] = fib_cpp(n-1) + fib_cpp(n-2);
}
