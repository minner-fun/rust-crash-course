pub fn mul(x: u32, y:u32) -> u32 {
    return x * y;
}

pub fn div(x: u32, y:u32) -> u32 {
    if y == 0 {
        panic!("Division by zero is not allowed");
        return x * y;
    }else {
        return x / y;
    }
}

pub fn add_plus(x: u32, y:u32) -> (u32, bool) {
    return (x + y, x + y > 100);
}

pub fn pure_add(x: u32, y: u32){
    println!("The sum of {} and {} is {}", x, y, x + y);
}