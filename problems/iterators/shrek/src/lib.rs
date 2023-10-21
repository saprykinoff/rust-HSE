#![forbid(unsafe_code)]

use std::usize;

// use itertools::Itertools;

pub fn count_lines_with_word(lines: impl Iterator<Item = String>, word: &str) -> usize {
    let mut ans: usize = 0;
    for line in lines {
        if line.to_lowercase().contains(&word.to_lowercase()) {
            ans += 1;
        }
    }
    ans
}

pub fn top_k_longest(
    lines: impl Iterator<Item = String>,
    k: usize,
) -> impl Iterator<Item = (usize, String)> {
    let mut ans = Vec::new();
    for (i, line) in lines.enumerate() {
        ans.push((-(line.len() as i32), i, line));
    }
    ans.sort();

    ans.into_iter().map(|x| (x.1, x.2)).take(k)
}

pub fn words_counter_iter<'a>(
    lines: impl Iterator<Item = String> + 'a,
    word: &'a str,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    // TODO: your code goes here.
    let mut ans = Vec::new();
    for (i, line) in lines.enumerate() {
        let x = line
            .to_lowercase()
            .match_indices(&word.to_lowercase())
            .count();
        if x > 0 {
            ans.push((x, i));
        }
    }
    ans.into_iter()
}
