
/// The number of fibonacci numbers to generate
const N: u32 = 123;

fn main() {
    let mut x: u128 = 0;
    println!("The N=0 factorial is {}", x);
    let mut y: u128 = 1;
    println!("The N=1 factorial is {}", y);
    let mut z = x + y;
    println!("The N=2 factorial is {}", z);
    for i in 3..N+1 {
        x = y;
        y = z;
        z = x + y;
        println!("The N={} factorial is {}", i, z);
    }
}
