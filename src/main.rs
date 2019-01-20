extern crate rayon;
extern crate num_cpus;

use std::env;

mod recursive;
mod iterator;
mod imperative;

fn pooler() -> rayon::ThreadPool {
    let cpus = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(cpus).build().unwrap();
    return pool
}

fn help() {
    println!("[HELP] You need to provide the calculation mode and the number to calculate the Fibonacci sum", );
    println!("\nModes are: [imperative, iterator, recursive]", );
    println!("$ fib <mode> <number>", );
    println!("\n$ fib iterator 10", );
    println!("The sum of Fibonnacci numbers of 10 is 231", );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 | 2 => help(),
        3 => {
            let mode: String = args[1].parse().expect("Error parsing mode argument");
            let number: u64 = args[2].parse().expect("Error parsing number argument");
            match mode.as_str() {
                "imperative" => {
                    println!("The sum of Fibonnacci numbers of {} is {}", number, imperative::fib(number));
                },
                "iterator" => {
                    println!("The sum of Fibonnacci numbers of {} is {}", number, iterator::fib(number as usize));
                },
                "recursive" => {
                    let pool = pooler();
                    let _n = pool.install(|| recursive::fib(number));
                    println!("The sum of Fibonnacci numbers of {} is {}", number, _n);
                },
                _ => help(),
            }
        }
        _ => help(),
    }
}
