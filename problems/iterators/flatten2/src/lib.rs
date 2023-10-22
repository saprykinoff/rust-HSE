#![forbid(unsafe_code)]

pub struct FlatMap<InIter, F, OutIter>
where
    OutIter: IntoIterator,
{
    inp_iter: InIter,
    f: F,
    out_iter: Option<OutIter::IntoIter>,
}

impl<InIter, F, OutIter> FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    OutIter: IntoIterator,
{
    fn new(outer: InIter, function: F) -> Self {
        FlatMap {
            inp_iter: outer,
            f: function,
            out_iter: None,
        }
    }
}

impl<InIter, F, OutIter> Iterator for FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: FnMut(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    type Item = OutIter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.out_iter.is_none() {
            if let Some(nw) = self.inp_iter.next() {
                self.out_iter = Some((self.f)(nw).into_iter());
            }
        }
        let iter = self.out_iter.as_mut()?;
        if let Some(res) = iter.next() {
            return Some(res);
        };
        if let Some(nw) = self.inp_iter.next() {
            self.out_iter = Some((self.f)(nw).into_iter());
        }
        self.out_iter.as_mut()?.next()
    }
}

pub fn flat_map<InputIterator, Mapping, OutputIterator>(
    iter: InputIterator,
    f: Mapping,
) -> FlatMap<InputIterator, Mapping, OutputIterator>
where
    InputIterator: Iterator,
    Mapping: FnMut(InputIterator::Item) -> OutputIterator,
    OutputIterator: IntoIterator,
{
    FlatMap::new(iter, f)
}
