pub struct LastRepeatIter<I>
where
    I: Iterator,
    I::Item: Copy,
{
    it: I,
    next: Option<I::Item>,
    last: bool,
}

impl<I> LastRepeatIter<I>
where
    I: Iterator,
    I::Item: Copy,
{
    pub fn new<It>(it: It) -> Self
    where
        It: IntoIterator<IntoIter = I, Item = I::Item>,
    {
        LastRepeatIter {
            it: it.into_iter(),
            next: None,
            last: false,
        }
    }
}

impl<I> Iterator for LastRepeatIter<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.last {
            match self.it.next() {
                None => self.last = true,
                Some(next) => self.next = Some(next),
            }
        }

        self.next
    }
}
