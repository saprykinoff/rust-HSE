#![forbid(unsafe_code)]

pub fn get_n_prime_numbers(n: u32) -> Vec<u32> {
    let mut sieve = vec![1; usize::try_from(n + 1).unwrap()];
    let mut ans = Vec::new();
    for i in 2..n {
        if sieve[i as usize] == 1 {
            ans.push(i);
            for j in (i * i..n).step_by(usize::try_from(n + 1).unwrap()) {
                sieve[j as usize] = 0;

            }
        }
    };
    ans



}
