#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct SplitString<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> SplitString<'a, 'b> {
    pub fn new(input: &'a str, delimiter: &'b str) -> Self {
        SplitString {
            remainder: Some(input),
            delimiter,
        }
    }
}

impl<'a, 'b> Iterator for SplitString<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(rem) = self.remainder else {
            return None;
        };
        if self.delimiter.is_empty() {
            if rem.is_empty() {
                self.remainder = None;
                return None;
            }
            let c = rem.chars().next().unwrap().len_utf8();
            self.remainder = Some(&rem[c..rem.len()]);
            return Some(&rem[0..c]);
        }

        // if rem.is_empty() {
        //     self.remainder = None;
        //     return Some(rem);
        // }
        let mut l = 0;
        let mut r = 0;
        let del_len: usize = self.delimiter.chars().map(|ch| ch.len_utf8()).sum();
        for ch in rem.chars() {
            r += ch.len_utf8();
            if r - l > del_len {
                l += 1;
                while !rem.is_char_boundary(l) {
                    l += 1;
                }
            }
            if r > rem.len() {
                break;
            }
            let tmp = &rem[l..r];
            if tmp == self.delimiter {
                self.remainder = Some(&rem[r..rem.len()]);
                return Some(&rem[0..l]);
            }
        }
        self.remainder = None;
        Some(rem)

        // fn move_l(i: i32) {
        //     l += rem[]
        // }
        // if self.delimiter.is_empty() {
        //     let res = &rem[0..1];
        //     if rem.len() == 1 {
        //         self.remainder = None
        //     } else {
        //         self.remainder = Some(&rem[1..rem.len()]);
        //     }
        //     return Some(res);
        // }

        // for i in self.delimiter.len()..=rem.len() {
        //     let f = i - self.delimiter.len();
        //     if &rem[f..i] == self.delimiter {
        //         let res = &rem[0..f];
        //         // if i == rem.len() {
        //         //     self.remainder = None;
        //         // } else {
        //         self.remainder = Some(&rem[i..rem.len()]);
        //         // }
        //         return Some(res);
        //     }
        // }
        // self.remainder = None;
        // Some(rem)
    }
}

pub fn split<'input, 'delimiter>(
    input: &'input str,
    delimiter: &'delimiter str,
) -> SplitString<'input, 'delimiter> {
    SplitString {
        remainder: Some(input),
        delimiter,
    }
}
