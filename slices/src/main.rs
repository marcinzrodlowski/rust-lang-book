fn main() {
    fn main() {
        let my_string = String::from("hello world");
    
        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s
        let word = first_word(&my_string);
    
        let my_string_literal = "hello world";
    
        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
    
        // Because string literals *are* string slices already, this works too, without the slice syntax
        let word = first_word(my_string_literal);
    }
}

fn first_word(s: &String) -> &str { // using string slices provied greater flexibility
    let bytes = s.as_bytes(); // array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iter returns each element in a collection and enumerate wraps the result of iter and returns each element as part of a tuple
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

