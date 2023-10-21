#![forbid(unsafe_code)]

pub fn add2(iterator: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    iterator.map(|x| x + 2)
}

struct Div3Iter {
    cur: i32,
}

impl Iterator for Div3Iter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 3;
        Some(self.cur)
    }
}

pub fn div3() -> impl Iterator<Item = i32> {
    Div3Iter { cur: 0 }
}

pub fn take_n(iterator: impl Iterator<Item = i32>, n: usize) -> Vec<i32> {
    let ans: Vec<_> = iterator.take(n).collect();
    ans
}
