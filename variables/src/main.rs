fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2; // shadowing, x = 12
    
    let spaces = "   ";
    let spaces = spaces.len(); // shadowing using different types

    let tup: (i32, f64, u8) = (500, 6.4, 1); // way of grouping the values (fixed length)
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let x = tup.0; // another way of getting the value from tuple

    let months: [&str; 2] = ["January", "February"]; // &str means that content is borrowed
    let a = [3; 5]; // [3,3,3,3,3]
}