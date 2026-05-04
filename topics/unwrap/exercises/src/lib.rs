pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let x:u32 = a.parse().expect("Failed to parse variable");
    let y:u32 = b.parse().expect("Failed to parse variable");
    x + y
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    let x = x.unwrap();
    let y = y.unwrap();
    x + y
}
