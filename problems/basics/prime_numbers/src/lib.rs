#![forbid(unsafe_code)]

fn is_prime(n:u32) -> bool {
    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {return false;}
    }
    true
}

pub fn get_n_prime_numbers(n: u32) -> Vec<u32> {
    let mut cur = 2;
    let mut ans = Vec::new();
    while ans.len() < n as usize {
        if is_prime(cur) {
            ans.push(cur);
        }
        cur += 1;
    }
    ans
}
