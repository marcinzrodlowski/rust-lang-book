fn main() {
    println!("Hello from main!");
    another_function(117);
    expression_example();
    returning_function();
    let y_minus_three = minus_three(5);
    println!("The value of y_minus_three is: {}", y_minus_three);
}

fn another_function(num: u32) {
    println!("The value of num is: {}", num); // 117
}

fn expression_example() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // without ending semicolon, otherwise it will be an statement
    };
    println!("The value of y is: {}", y); // 4
}

fn returning_function() -> u32 {
    let z = 2;
    z // implicit return
}

fn minus_three(x: i32) -> i32 {
    x - 3
}