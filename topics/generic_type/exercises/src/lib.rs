pub fn first<A, B>(t: (A, B)) -> A {
    t.0
}

pub fn last<A, B>(t: (A, B)) -> B {
    t.1
}

#[derive(Debug)]
pub struct Rectangle <T = u32>{
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}
