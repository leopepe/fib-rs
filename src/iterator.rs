struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
	    Fibonacci { curr: 1, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

pub fn fib(n: usize) -> u64 {
    let mut sum: u64 = 0;
    for i in Fibonacci::new().take(n) {
        sum+=i
    }
    return sum
}
