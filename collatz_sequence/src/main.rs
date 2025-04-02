/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    // while n != 1 {
    //     if n % 2 == 0 {
    //         n = n / 2;
    //         length += 1;
    //     } else {
    //         n = 3 * n + 1;
    //         length += 1;
    //     };
    // }
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
    // dbg!(n);
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
