fn main() {
    hello();
    hello_via_macro();
    play_with_strings()
}

fn hello() {
    let name: &str = "baris";
    let surname: &str = "alis";
    let year: i32 = 2025;

    println!("Hello, {}", name); // Hello, baris
    println!("Hello, {name} {surname}"); // Hello, baris alis
    println!("Hello, {param1} {param2}", param1 = name, param2 = year); // Hello, baris 2025
    assert_eq!(format!("Hello, {name}", name = year), "Hello, 2025");
}

// - - - - - - - - -

macro_rules! say_hello_to {
    ($name:expr) => {
        println!("Hello, {}", $name)
    };
}

fn hello_via_macro() {
    say_hello_to!("jupiter");
}

// - - - - - - - - -

fn play_with_strings() {
    let str1: &str = "string1";
    let str1_1 = "string1_1";
    // string1 = "sthElse"; // ❌ Because string1 is immutable
    assert_eq!(str1, "string1");
    assert_eq!(str1_1, "string1_1");

    let mut str_ref: &str = "hello";
    assert_eq!(str_ref, "hello");
    str_ref = "world"; // ✅ This works. It just points to a different string literal
    assert_eq!(str_ref, "world");
    // str_ref.push_str(" there");  // ❌ This would NOT work - can't modify the content

    let mut owned_string = String::from("hello");
    for i in 0..5 {
        owned_string.push_str(i.to_string().as_str()); // ✅ This works - can modify content
    }
    assert_eq!(owned_string, "hello01234");
    owned_string = String::from("completely new"); // ✅ Can also reassign
    assert_eq!(owned_string, "completely new");

    /*
    - `&str` is like a pointer to a read-only text in a book
    - `String` is like having your own notepad where you can write and modify text
    */
}
