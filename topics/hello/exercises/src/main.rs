// Attribute that tells the Rust compiler to suppress warnings
// about unused code
// Attributes - metadata for the compiler
#![allow(unused)]

// main() is the entry point of Rust program

// println! is a macro that prints text to the console.
// Macros in Rust generate code at compile time and are
// invoked with an exclamation mark (!).
const NUM: u32 = 123;
fn main(){
    println!("hello Rust!");
    let x = 12;
    println!("x is {x}");
    const Y: i32 = 1;
    let z : i32 = Y + x; 
    println!("Y is {}", Y);
    println!("NUM is {NUM}");
    println!("{0} + {1} = {2}", Y, x, z );
    
    let mut name = "Minner";
    name = "minmin";
    println!("name is {name}");

    let point = (3,4);
    println!("DEBUG tuple: point is {:?}", point);
    println!("DEBUG pretty tuple: point is {:#?}", point);

    let age: _ = 30;
    println!("age is {age}");
    let age: u32 = 50;
    println!("age is {age}");
    let age: bool = true;
    println!("age is {age}");

}