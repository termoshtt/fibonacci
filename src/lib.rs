const MAX_N: usize = 1000;

struct FibMemo {
    memo: [usize; MAX_N],
}

impl FibMemo {
    fn new() -> Self {
        FibMemo { memo: [0; MAX_N] }
    }

    fn calc(&mut self, n: usize) -> usize {
        if n < 2 {
            return n;
        }
        if self.memo[n] != 0 {
            return self.memo[n];
        }
        self.memo[n] = self.calc(n - 2) + self.calc(n - 1);
        self.memo[n]
    }
}

pub fn fib_memo(n: usize) -> usize {
    let mut f = FibMemo::new();
    f.calc(n)
}

pub unsafe fn fib_static(n: usize) -> usize {
    static mut MEMO: [usize; MAX_N] = [0; MAX_N];
    if n < 2 {
        return n;
    }
    if MEMO[n] != 0 {
        return MEMO[n];
    }
    MEMO[n] = fib_static(n - 2) + fib_static(n - 1);
    MEMO[n]
}

extern "C" {
    pub fn fib_cpp(n: usize) -> usize;
}
