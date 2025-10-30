#![allow(unused)]

fn rustlatin(sentence: &str) -> String {
    //                          ^^^^^^^
    // The correct return type needs to be added by you,
    // depending on what the vector's exact type is.

    let mut collection_of_words: Vec<String> = Vec::new();
    //                         ^^^^^^^^^^^^
    // When you first open this file RA is not able to infer
    // the type of this vector. Once you do the implementation,
    // the type should appear here automatically.

    // Your implementation goes here:
    // Iterate over the sentence to split it into words.
    // Push the words into the vector.
    // Correct the return type of the vector
    for word in sentence.split(' ') {
        if word.starts_with(&['a', 'e', 'i', 'o', 'u']) {
            collection_of_words.push(format!("sr{}", word));
        } else {
            collection_of_words.push(format!("{}rs", word));
        }
    }

    collection_of_words.join(" ")
}

/// adds prefix "sr" and suffix "rs" according to the rules
fn latinize(s: &str) -> String {
    // You need to add the right arguments and return type, then implement
    // this function.
    rustlatin(s)
}

#[test]
fn test_latinizer() {
    // Uncomment these test cases
    assert_eq!(latinize("rust"), "rustrs");
    assert_eq!(latinize("helps"), "helpsrs");
    assert_eq!(latinize("you"), "yours");
    assert_eq!(latinize("avoid"), "sravoid");
}

#[test]
fn correct_translation() {
    // Why can we compare `&str` and `String` here?
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#impl-PartialEq%3C%26%27a%20str%3E

    // Uncomment this:
    assert_eq!(
        "rustrs helpsrs yours sravoid sra lotrs srof srirritating bugsrs",
        rustlatin("rust helps you avoid a lot of irritating bugs")
    )
}
