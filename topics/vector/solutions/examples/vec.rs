#![allow(unused)]

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut vv: Vec<u32> = Vec::new();
    v.push(-1);
    v.push(-2);
    v.push(-3);
    println!("vec: {:?}", v);
    println!("len: {}", v.len());

    // vec! macro
    let v: Vec<i32> = vec![-1, -2, -3];

    // Vec<u8>
    let v = vec![1u8, 2, 3, 4];
    println!("vec: {:?}", v);
    // Initialize with all 5 elements equal to 1
    let v = vec![1u8; 5];
    println!("vec: {:?}", v);

    // Get
    let x = v[1];
    println!("v[1]: {}", x);

    let x: Option<&u8> = v.get(1);
    println!("get: {:?}", x);

    // Update
    let mut v = vec![1, 2, 3];
    v[1] = 10;

    // pop - remove last element
    let mut v = vec![1, 2];
    let x: Option<i32> = v.pop();
    println!("pop: {:?}", x);

    // Slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..3];
    println!("slice: {:?}", s);
}
