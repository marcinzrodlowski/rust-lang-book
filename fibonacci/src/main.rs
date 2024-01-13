use std::io;

fn main() {
    println!("What fibonacci index would you like?");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    // shadowing
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please input a valid number.");
        }
    };

    let result = fibonacci(n);
    println!("The fibonacci value on position {} is {}", n, result);
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if n == 0 {
        return 0;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}