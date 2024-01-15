fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a


}

fn main() {
    let n = 46;
    println!("Fibonacci number at position {} is: {}", n, fibonacci_iterative(n));
}
