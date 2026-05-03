// // Option<T> = Some(T) | None
// let x: Option<i32> = Some(-1);
// let x: Option<i32> = None;
// println!("{:?}", x);

// // Result<T, E> = Ok(T) | Error(E)
// let res: Result<i32, &str> = Ok(100);
// let res: Result<i32, &str> = Err("error 💀");
// println!("{:?}", res);

#[derive(Debug, PartialEq)]
enum Command{
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize{width: u32, height: u32},
}

fn main(){

    let play_cmd: Command = Command::Play;
    let stop_cmd: Command = Command::Stop;
    let skip_cmd: Command = Command::Skip(60);
    let Resize_cmd: Command = Command::Resize{width:500, height:300};

    println!("Result: {:?}: ", Resize_cmd);

    println!("play_cmd == stop_cmd ? {}", play_cmd == stop_cmd);

    let x: Option<i32> = Some(-1);
    println!("Option: {:?}", x);

    let res: Result<i32, &str> = Ok(100);
    println!("Result: {:?}", res);
    

}