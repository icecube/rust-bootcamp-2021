
/// The number to test
const N: u128 = 1651894653;

fn is_divisble(n: u128) -> bool {
    for i in 2..n/2 {
        if n%i == 0 {
            return true;
        }
    }
    false
}

fn main() {
    if is_divisble(N) {
        println!("N={} is NOT prime", N);
    } else {
        println!("N={} IS prime", N);
    }
}
