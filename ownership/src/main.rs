mod ownership_and_functions;
use ownership_and_functions::ownership_and_functions;

fn main() {
    let s = "hello"; // The reference (&str) itself, being a pointer, is stored on the stack but the actual data is stored on the heap
    let text = String::from("hello"); // this string CANNOT be mutated, stored on the heap
    let mut text2 = String::from("hello"); // this string CAN be mutated and is also stored on the heap
    text2.push_str(", world!");
    println!("{}", text2);

    // move:
    let s1 = String::from("hello"); // stack holds pointer to the memory, length (1char = 1 byte) and capacity but content is stored on the heap
    let s2 = s1; // here we are moving the pointer, the length and capacity from the stack (we DO NOT moving the data on the heap), moreover now s1 CANNOT be used anymore (this mechanism prevents from double free memory error when scope is closed). It's called move so s1 was moved into s2. Rust will never create 'deep copies' of data.

    // clone:
    let s3 = String::from("hello");
    let s4 = s3.clone(); // creates 'deep copy' (copy the heap data not just the stack data like in move).

    //copy (stack-only data):
    let x = 5;
    let y = x; // in this particular case, x is still valid because integers have a known size at compile time and stored entirely on the stack thus copies are quick to make. There is no difference between deep and shallow copy then.
    // Copy types:
    // all integers
    // boolean
    // all floats
    // characters
    // tuples (if they only contain types that are also Copy)

    ownership_and_functions();

} // this scope is now over, memory is freed. Rust calls function called 'drop' at this place automatically to release memory.

// ownership rules:
// the length of heap is unknown/flexible also heap works with pointers to select certain amount of space and allocate data there
// the length of stack is known and fixed, faster than heap (no pointers)
// each value in rust has a variable that's called its owner
// there can only be one owner at a time
// when the owner goes out of scope, the value will be dropped