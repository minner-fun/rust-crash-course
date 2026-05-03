#![allow(unused)]

fn main()-> (){
    let _msg:String = String::from("hello");
    let _msg:String = "world".to_string();
    println!("msg: {}", _msg);
    let s: &str = "hello world";
    println!("s: {}", s);
    let msg: String = s.to_string();
    let s: &str = &msg[..3];
    println!("s: {}", s);

    let mut msg: String = "hello world".to_string();
    msg.push_str("!!!");
    msg += "how are you?";
    println!("msg: {}", msg);

    let name = "Rust";
    let version = 1.88;
    let emoji = "🚀🐕";
    let info = format!("learn {} version {} is fun {}", name, version, emoji);
    println!("info: {}", info);
}