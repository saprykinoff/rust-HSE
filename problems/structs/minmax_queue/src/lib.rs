#![forbid(unsafe_code)]

#[derive(Default)]
pub struct MinMaxQueue {
    stl: Vec<(i32, i32, i32)>,
    str: Vec<(i32, i32, i32)>,
}

impl MinMaxQueue {
    pub fn new() -> Self {
        MinMaxQueue {
            stl: Vec::new(),
            str: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        if self.stl.is_empty() {
            self.stl.push((value, value, value));
            return;
        }
        let last = self.stl.last().unwrap();
        self.stl.push((
            value,
            std::cmp::min(value, last.1),
            std::cmp::max(value, last.2),
        ));
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.str.is_empty() {
            while let Some(element) = self.stl.pop() {
                if self.str.is_empty() {
                    self.str.push((element.0, element.0, element.0));
                } else {
                    let last = self.str.last().unwrap();
                    self.str.push((
                        element.0,
                        std::cmp::min(element.0, last.1),
                        std::cmp::max(element.0, last.2),
                    ));
                }
            }
        }

        if self.str.is_empty() {
            None
        } else {
            Some(self.str.pop().unwrap().1)
        }
    }

    pub fn first(&self) -> Option<i32> {
        if !self.str.is_empty() {
            Some(self.str.last().unwrap().0)
        } else if !self.stl.is_empty() {
            Some(self.stl.first().unwrap().0)
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<i32> {
        if !self.stl.is_empty() {
            Some(self.stl.last().unwrap().0)
        } else if !self.str.is_empty() {
            Some(self.str.first().unwrap().0)
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        if self.stl.is_empty() {
            return Some(self.str.last().unwrap().1);
        }
        if self.str.is_empty() {
            Some(self.stl.last().unwrap().1)
        } else {
            Some(std::cmp::min(
                self.stl.last().unwrap().1,
                self.str.last().unwrap().1,
            ))
        }
    }

    pub fn max(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        if self.stl.is_empty() {
            return Some(self.str.last().unwrap().2);
        }
        if self.str.is_empty() {
            Some(self.stl.last().unwrap().2)
        } else {
            Some(std::cmp::max(
                self.stl.last().unwrap().2,
                self.str.last().unwrap().2,
            ))
        }
    }

    pub fn len(&self) -> usize {
        self.stl.len() + self.str.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stl.is_empty() && self.str.is_empty()
    }
}
