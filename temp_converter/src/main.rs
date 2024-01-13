use std::io;

fn main() {
    println!("Please input a temperature in fahrenheit");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    // shadowing
    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please input a number");
        }
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}F is equal to {}C", fahrenheit, celsius);
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}