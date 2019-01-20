# fib

A try-out using Rust to calculate the Fibonacci sum using three methods: Imperative, Recursiveness and Iterator.

# Performance

## Iterator

```shell
λ time cargo run -- iterator 90
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/fib iterator 90`
The sum of Fibonnacci numbers of 90 is 12200160415121876736

real    0m0.048s
user    0m0.030s
sys     0m0.018s
```

## Imperative

```shell
λ time cargo run -- imperative 90
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/fib imperative 90`
The sum of Fibonnacci numbers of 90 is 2880067194370816120

real    0m0.056s
user    0m0.028s
sys     0m0.028s
```

## Recursive

Note: The recursive mode needs to use a memorization method in due to avoid recalculating all the numbers before over-and-over again. The inner function acumulates the previous sum. It is also more performatic if rayon pooler is used to spawn threads for each calculation (max of 2x number of cores)

```shell
λ time cargo run -- recursive 90
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/fib recursive 90`
The sum of Fibonnacci numbers of 90 is 2880067194370816120

real    0m0.061s
user    0m0.038s
sys     0m0.024s
```