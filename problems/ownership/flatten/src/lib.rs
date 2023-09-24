#![forbid(unsafe_code)]

pub fn flatten<const N: usize>(data: Vec<Box<[&mut i32; N]>>) -> Vec<&mut i32> {
    let mut ans: Vec<&mut i32> = Vec::new();
    for x in data {
        for el in *x {
            ans.push(el)
        }
    }
    ans
}

pub fn transform_to_fixed_arr<const N: usize>(data: &mut Vec<Vec<i32>>) -> Vec<Box<[&mut i32; N]>> {
    let mut ans: Vec<Box<[&mut i32; N]>> = Vec::new();
    for x in data {
        if x.len() != N {
            panic!("Inner vectors are of different size");
        }
        let mut tmp: Vec<&mut i32> = Vec::new();
        for el in x {
            tmp.push(el)
        }
        ans.push(Box::new(tmp.try_into().unwrap()));
    }
    ans
}
