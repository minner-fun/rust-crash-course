pub fn zeros() -> [u32; 100] {
    let arr: [u32; 100] = [0; 100];
    return arr;
}

pub fn first_3(s: &[u32]) -> &[u32] {
    let s: &[u32] = &s[..3];
    return s;
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let long = s.len();
    let s: &[u32] = &s[long-3..];
    return s;
}
