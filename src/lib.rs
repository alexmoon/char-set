use std::ops::RangeInclusive;

use range_set;
use range_set::RangeSet;

pub struct CharSet {
    range_set: RangeSet<[RangeInclusive<u32>; 1]>,
}

impl CharSet {
    pub fn new() -> Self {
        CharSet {
            range_set: RangeSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        CharSet {
            range_set: RangeSet::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.range_set.is_empty()
    }

    pub fn clear(&mut self) {
        self.range_set.clear()
    }

    pub fn into_inner(self) -> RangeSet<[RangeInclusive<u32>; 1]> {
        self.range_set
    }

    pub fn insert(&mut self, element: char) -> bool {
        self.range_set.insert(element as u32)
    }

    pub fn remove(&mut self, element: char) -> bool {
        self.range_set.insert(element as u32)
    }

    pub fn insert_range(&mut self, range: RangeInclusive<char>) {
        let (start, end) = range.into_inner();
        if (end as u32) >= 0xe000 && (start as u32) < 0xd800 {
            // Unicode code points 0xd800 - 0xdfff are not valid unicode scalar values.
            // Therefore, if the range spans that set of code points we must cut them
            // out to avoid having invalid unicode scalar values in the set.
            self.range_set.insert_range((start as u32)..=0xd7ff);
            self.range_set.insert_range(0xe000..=(end as u32));
        } else {
            self.range_set.insert_range((start as u32)..=(end as u32));
        }
    }

    pub fn remove_range(&mut self, range: RangeInclusive<char>) {
        let (start, end) = range.into_inner();
        self.range_set.remove_range((start as u32)..=(end as u32));
    }

    pub fn iter<'a>(&'a self) -> CharIter<'a> {
        CharIter {
            inner: self.range_set.iter(),
        }
    }

    pub fn ranges<'a>(&'a self) -> RangeIter<'a> {
        RangeIter {
            inner: self.range_set.ranges(),
        }
    }
}

impl From<RangeInclusive<char>> for CharSet {
    fn from(range: RangeInclusive<char>) -> Self {
        let (start, end) = range.into_inner();
        CharSet {
            range_set: ((start as u32)..=(end as u32)).into(),
        }
    }
}

pub struct CharIter<'a> {
    inner: range_set::Iter<'a, [RangeInclusive<u32>; 1], u32>,
}

impl<'a> Iterator for CharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // We ensure only valid unicode scalar values can be inserted into
        // the range_set through our API and there is no public access to
        // the underlying range_set without consuming the CharSet, so
        // char::from_u32_unchecked() is safe.
        self.inner
            .next()
            .map(|c| unsafe { std::char::from_u32_unchecked(c) })
    }
}

pub struct RangeIter<'a> {
    inner: std::slice::Iter<'a, RangeInclusive<u32>>,
}

impl<'a> Iterator for RangeIter<'a> {
    type Item = RangeInclusive<char>;

    fn next(&mut self) -> Option<Self::Item> {
        // We ensure only valid unicode scalar values can be inserted into
        // the range_set through our API and there is no public access to
        // the underlying range_set without consuming the CharSet, so
        // char::from_u32_unchecked() is safe.
        self.inner.next().map(|r| unsafe {
            std::char::from_u32_unchecked(*r.start())..=std::char::from_u32_unchecked(*r.end())
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, *('a'..='b').start() as u32);
    }
}
