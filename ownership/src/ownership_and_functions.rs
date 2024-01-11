pub fn ownership_and_functions() {
    let s = String::from("hello"); // heap

    take_ownership(s); // If value moves to the function then is no longer valid in the scope (compile-time error)

    let x = 5; // stack

    makes_copy(x); // but if Copy value goes to a function then it's okay to still use it.

    let s1 = gives_ownership(); // returned value goes into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into fn which also returns value into s3

    let (s4, len) = calculate_length(s3); // we can return multiple values from a function

    let len_of_s4 = calculate_length_with_reference(&s4); // & allow to refer to some value without taking ownership of it and because of that the value it points to will not be dropped when the reference goes out of scope. References are immutable by default and we can have many of them at the same scope. MUST ALWAYS BE VALID.
    println!("The length of '{}' is: {}", s4, len_of_s4);

    let mut some_text = String::from("Hiho");
    change(&mut some_text); // it's a mutable reference thus we CAN modify it inside a function but there is a catch, you can have only ONE mutable reference in a scope also we cannot mix mutable and immutable references together (except when mutable reference was used already then mix with immutable will work). Thanks to that it prevent 'data races'. If we really need to use it more than once we can create simple scope for that -> { and use it inside }
    println!("Some_text: {}", some_text);

    // let reference_to_nothing = dangle(); // error
    let result_of_no_dangle = no_dangle();
} // here s3 goes out of scope and is dropped, s2 goes out of scope but was moved so nothing happens, s1 is dropped

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// this function will move its return value into the function that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("hello");
    some_string // implicit return without ; some_string is returned and moves out to the calling function
}

// take a string and return one
fn takes_and_gives_back(a_string: String) -> String {
    a_string // returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!")
}

// fn dangle() -> &String {
//     let s = String::from("dangle");
//     &s // at this point &s will be lost so reference will point to nothing
// }

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
}