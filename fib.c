#define MAX_N (1000)

int memo[MAX_N + 1];

int fib_c(int n) {
    if (n <= 1) return n;
    if (memo[n] != 0) return memo[n];
    return memo[n] = fib_c(n-1) + fib_c(n-2);
}
