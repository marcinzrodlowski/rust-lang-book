fn main() {
    println!("{}", greater_than_five(7));
    divisible_by(4);

    let custom_value = 7;
    let is_positive = if custom_value >= 0 { true } else { false }; // similar to ternary operator in JS
    println!("The value of is_positive is: {}", is_positive);
    loop_example();
    countdown();
    for_loop_example();
    for_with_range();
}

fn greater_than_five(num: i32) -> bool {
    if num > 5 {
        true
    } else {
        false
    }
}

fn divisible_by(number: u32) {
    match number {
        _ if number % 4 == 0 => println!("number is divisible by 4"),
        _ if number % 2 == 0 => println!("number is divisible by 2"),
        _ => println!("who knows..."),
    }
}

fn loop_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result); // 20
}

fn countdown() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn for_with_range() {
    for number in (1..5).rev() {
        println!("{}!", number);
    }
}