#![forbid(unsafe_code)]

pub fn is_palindrome(number: u32) -> bool {
    let mut x = number;
    let mut y:u64 = 0;
    while x > 0 {
        let d = x % 10;
        x = x / 10;
        y = y * 10 + d as u64;
    }
    y == number as u64
}
