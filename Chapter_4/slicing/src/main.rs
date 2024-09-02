// slicing is basically referencing so it does not have ownership over the variable that it's slicing
// you may also take slices of literals

fn main() {
    let a = [1,2,3,4,5];

    let slice = &a[1..3] // works the same way as strings do!

    assert_eq!(slice, &[2,3]);

    let s = String::from("Hello World");

    /* 
    let hello = &s[..5]; // same as [0..5] // [starting_index..ending_index] ending_index is one more than the last position in the slice
    let world = &s[6..]; // same as [6..11]
    */

    let word = first_word(&s);

    //s.clear(); // error, s needs to be mutable

    println!("the first word is: {word}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

}

/* 
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Converts the string into an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead
                                                 // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.
        if item == b' ' {
            return i; // If we find a space, we return the position. Otherwise, we return the length of the string by using s.len().
        }
    }

    s.len()
}
*/

fn first_word(s: &str) -> &str { // This both works on literals and non-literals!
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}