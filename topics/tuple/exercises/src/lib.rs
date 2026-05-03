pub fn first(t: (bool, u32, char)) -> bool {
    t.0
}

pub fn last(t: (bool, u32, char)) -> char {
    t.2
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    let mut tmp: u32 = t.0;
    let mut t = t;
    t.0 = t.1;
    t.1 = tmp;
    t
}
