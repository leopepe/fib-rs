extern crate rayon;
extern crate num_cpus;

fn pooler() -> rayon::ThreadPool {
    let cpus = num_cpus::get() * 2;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(cpus).build().unwrap();
    return pool
}

pub fn fib(n: u64) -> u64 {
    let _pool = pooler();
    fn inner(n: u64, penultimate: u64, last: u64) -> u64 {
        match n {
            0 => penultimate,
            1 => last,
            _ => inner(n - 1, last, penultimate + last),
        }
    }
    _pool.install(|| inner(n, 0, 1))
}