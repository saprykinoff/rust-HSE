#![forbid(unsafe_code)]

pub fn get_nth_fibonacci(n: u32) -> u32 {
    if n < 1 {return 0;}
    if n == 1 {return 1;}
    get_nth_fibonacci(n - 1) + get_nth_fibonacci(n - 2)
}
