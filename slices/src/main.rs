#![allow(unused)]

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];    // returns string slice from starting_index to 'i' if space is found
        }
    }

    &s[..]  // returns whole string if space is not found
}

fn main() {
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
-----------------------------------------------

Following section is for slice syntax reference

-----------------------------------------------
*/

fn main1() {
    let s = String::from("hello world");

    let hello = &s[0..5];   // slice syntax -> [starting_index..ending_index]
    let world = &s[6..11];
}

fn main2() {
    let s = String::from("hello");

    let slice = &s[0..2];   
    let slice = &s[..2];    // these are equal
}

fn main3() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];    // these are also equal
}

fn main4() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..]; // these again, are equal
}

fn other_slice_types() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}


