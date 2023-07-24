use std::io;

// generate the nth fibonacci number
fn main() {
    println!("Enter n to calculate nth Fibonacci number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: i64 = input.trim().parse().unwrap();

    match number {
        1 => println!("{number}st fibonacci number: {}", fib_n(number)),
        2 => println!("{number}nd fibonacci number: {}", fib_n(number)),
        3 => println!("{number}rd fibonacci number: {}", fib_n(number)),
        i if i >= 4 => println!("{number}th fibonacci number: {}", fib_n(number)),
        _ => println!("Error, invalid input"),
    }
}

// Let 0 be the 'first' fib-num, 1 be 'second', 1 be 'third', 2 be 'fourth', etc.
fn fib_n(x: i64) -> i64 {
    if x <= 0 {
        -1
    } else if x == 1 {
        0
    } else if x == 2 {
        1
    } else {
        fib_n(x - 1) + fib_n(x - 2)
    }
}
