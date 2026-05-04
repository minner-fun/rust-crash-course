#![allow(unused)]

fn main() {
    // 1. Each value in Rust has a variable that’s called its owner.

    // 2. There can only be one owner at a time
    let s = String::from("dog");
    // Owner of the string data "dog" is s

    let s1 = s;
    // Ownership of "dog" is now moved to s1.
    // 's' is no longer valid.

    let s2 = s1;
    // Ownership of "dog" is now moved to s2.
    // 's1' is no longer valid.

    println!("{}", s2); // This will print "dog"
}