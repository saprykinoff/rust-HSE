#![forbid(unsafe_code)]

pub fn where_k_th_ordinal_element_greater<'a>(
    lhs: &'a Vec<i32>,
    rhs: &'a Vec<i32>,
    k: usize,
) -> &'a Vec<i32> {
    let mut tmpl = lhs.clone();
    let mut tmpr = rhs.clone();
    tmpl.sort();
    tmpr.sort();
    if tmpl[k] <= tmpr[k] {
        rhs
    } else {
        lhs
    }
}
