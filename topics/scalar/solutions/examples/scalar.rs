#![allow(unused)]

// Scalar types represent a single value
fn exe(){
    let a8 :i8 = -8;
    let a16: i16 = -16;
    let a32: i32 = 0;
    let a64: i64 = -86; 
    let aa: i8 = i8::MIN;
    let ab: i8 = i8::MAX;
    println!("Signed integers: {}, {}, {}, {}", a8, a16, a32, a64);
    println!("i8 min: {}, max: {}", aa, ab);


    let b8 : u8 = 8;
    let b16: u16 = 16;
    let b32: u128 = 256;
    // let b64: u512 = 512;
    // println!("Unsigned integers: {}, {}, {}, {}", b8, b16, b32, b64);

    let ba: u16 = u16::MIN;
    let bb: u16 = u16::MAX;
    println!("u16 min: {}, max:{}", ba,bb);

    println!("Unsigned integers: {}, {}", b8, b16);

    let c32: f32 = 3.14;
    let c64: f64 = 2.71828;
    println!("Floating point numbers: {}, {}", c32, c64);

    let d: bool = true;
    println!("Boolean: {}", d);

    let e: char = 'e';
    let f: String = String::from("Hello");
    let ff: &str = "1234";
    println!("Character: {}, String: {}, ff: {}", e, f, ff);

    let g: isize = -45;
    let h: usize = 64;
    println!("isize: {}, usize: {}", g, h);

    let j: i32 = bb as i32;
    println!("Cast u16 max to i32: {}", j);

    // let k:bool = a32 as bool;
    // println!("Cast i32 to bool: {}", k);
    // let ffg: u32 = ff as u32;
    // println!("Cast &str to u32: {}", ffg);

    let l: u64 = u64::MAX;
    let ll: usize = usize::MAX;
    let ll: u64 = ll as u64;
    println!("l equals ll: {}", l == ll);

    let  c : char = 'c' ;
    let  z : char = 'ℤ' ;
    let  heart : char = '❤' ;
    let  e : char = '🦀' ; // 表情符号是有效的字符值
    println!("Characters: {}, {}, {}, {}", c, z, heart, e);

    let da: i32 = -1;
    let db: u32 = da as u32;
    println!("Cast i32 to u32: {}", db);

    let dc: f32 = 3.2;
    let dd:i32 = dc as i32;
    println!("Cast f32 to i32: {}", dd);



}



fn main() {
    // Signed integers
    // Range: -(2^(n-1)) to 2^(n-1) - 1
    let i0: i8 = -1;
    let i1: i16 = 2;
    let i2: i32 = 3;
    let i3: i64 = -4;
    let i4: i128 = 5;
    // Depends on computer architecture
    let i5: isize = -6;

    // Unsigned integers
    // 0 to 2^n - 1
    let u0: u8 = 1;
    let u1: u16 = 2;
    let u2: u32 = 3;
    let u3: u64 = 4;
    let u4: u128 = 5;
    // Depends on computer architecture
    let u5: usize = 6;

    // Floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Character
    // Declared with single quote
    let c: char = 'c';

    // Type conversion
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("i32: {} to u32: {}", i, u);

    // Min and max
    let max = i32::MAX;
    let min = i32::MIN;
    let max_u = isize::MAX;
    println!("i32 min: {}, max: {}, max_u: {}", min, max, max_u);

    // // Overflow
    // let mut u: u32 = u32::MAX;
    // u += 1;
    // // Overflow doesn't panic when compiled with --release
    // println!("u32 silent overflow: {}", u);

    // Return None on overflow
    println!("u32 check overflow: {:?}", u32::checked_add(u32::MAX, 1));

    // Explicitly allow overflow
    println!("u32 allow overflow: {}", u32::wrapping_add(u32::MAX, 1));

    exe();
}
