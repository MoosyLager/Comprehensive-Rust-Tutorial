fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
        // return todo!("Implement this");
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
        // return todo!("Implement this");
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
