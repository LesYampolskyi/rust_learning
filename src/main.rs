fn main() {
    let s = String::from("Hello world");

    let word = first_word(&s);
    println!("{}", word);

    let hello = &s[0..5];
    let world = &s[6..];
    println!("Slice one: {}", hello);
    println!("Slice two: {}", world);

    let slice_string = first_word_slice(&s);

    println!("Slice is: {}", slice_string);

    println!("{word}");
    println!("Slice is: {}", slice_string);

    let a = first_word_for_slice(&s);
    let b = first_word_for_slice(&s[..]);
    let c = first_word_for_slice(&s[0..5]);
    // let c = first_word_for_slice(s); === Error

    println!("for slice: {}, {}, {}", a, b, c);

    let string_literal = "hello world";
    let d = first_word_for_slice(string_literal);
    let e = first_word_for_slice(&string_literal);

    println!("string literal: {}, {}", d, e);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word_for_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
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
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
