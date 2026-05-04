pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for num in nums.iter() {
        sum += num;
    }
    return sum;
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    vec![i; n]
}
