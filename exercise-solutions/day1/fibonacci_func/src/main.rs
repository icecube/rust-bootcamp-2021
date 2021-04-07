
/// The number of fibonacci numbers to generate
const N: u32 = 123;

fn fib(n: u128) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    for i in 0..N+1 {
        println!("The N={} factorial is {}", i, fib(i.into()));
    }
}
