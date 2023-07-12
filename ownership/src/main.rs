fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    // invalids s1 such that no double free error can occur
    let s2 = s1;

    let s1 = String::from("world");
    // makes a copy of the actual heap contents
    let s2 = s1.clone(); 

    let s1 = String::from("madada");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // references and borrowing
    let mut s1 = String::from("manama");

    // action of creating a reference
    // borrowing
    let len = calculate_length_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable borrow
    append_suffix(&mut s1);

    println!("String with suffix: {}.", s1);

    // mutable and immutable together is not allowed
    // scope end of references is last used
    // Non-Lexical Lifetimes (NLL)

    // slice type
    // LOL PYTHON but in a system language
    // reference contiguous sequence of elements in a collection
    // rather then the whole collection
    let s = String::from("hello world");

    let hello = &s[0..5];
    // same as
    let hello = &s[..5];
    let world = &s[6..11];

    let len = s.len();

    let slice = &s[3..len];
    // same as
    let slice = &s[3..];

    // takes whole string
    let slice = &s[..];
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    // String as bytes array
    let bytes = s.as_bytes();

    // iter: iterate over elements
    // enumerate: wrap each element in a (idx, elem) tuple
    // pattern matching to get idx (i) and elem (item)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn append_suffix(s: &mut String) {
    s.push_str("suffix");
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}