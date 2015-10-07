//! Advanced iterator operations.
//!

use std::fmt::{ Display, Write };

pub trait AdvancedIterator: Iterator {
    /// Returns an iterators of iterators (chunks), where each subiterator
    /// iterates over a `size` number of elements from the original iterator.
    /// The last chunk may be shorter, if there are not enough elements in the
    /// original iterator.
    fn chunk(self, size: usize) -> Chunk<Self>
        where Self: Sized
    {
        Chunk{ inner: self, size: size }
    }

    /// Alternate elements from every iterator in the original iterator until
    /// the first one which runs out of elements.
    fn interleave(self) -> Interleave<Self::Item>
        where Self: Sized, Self::Item: Iterator
    {
        let inner = self.collect::<Vec<_>>();
        assert!(inner.len() > 0);

        Interleave{ inner: inner, index: 0 }
    }

    /// Transpose an iterator of iterators. The outer iterator must be finite
    /// and additionally implement ExactSizeIterator.
    fn transpose(self) -> Chunk<Interleave<Self::Item>>
        where Self: Sized + ExactSizeIterator, Self::Item: Iterator
    {
        let len = self.len();
        self.interleave().chunk(len)
    }

    /// Chains all the iterators in the original iterator into a single
    /// iterator.
    fn chain_all(self) -> ChainAll<Self>
        where Self: Sized, Self::Item: Iterator
    {
        ChainAll { current: None, all: self }
    }

    /// Join the elements in the iterator into a `String` separated by
    /// `separator`.
    fn join(&mut self, separator: &str) -> String
        where Self::Item: Display
    {
        let mut result = String::new();

        if let Some(first) = self.next() {
            write!(&mut result, "{}", first).unwrap();

            while let Some(item) = self.next() {
                write!(&mut result, "{}{}", separator, item).unwrap();
            }
        }

        result
    }
}

impl<I> AdvancedIterator for I where I: Iterator {}

#[test]
fn chunk() {
    let input  = ["foo", "bar", "baz", "qux", "quux"];
    let mut chunks = input.iter().chunk(2);

    let mut chunk = chunks.next().unwrap();
    assert_eq!(chunk.next().unwrap(), &"foo");
    assert_eq!(chunk.next().unwrap(), &"bar");
    assert_eq!(chunk.next(), None);

    let mut chunk = chunks.next().unwrap();
    assert_eq!(chunk.next().unwrap(), &"baz");
    assert_eq!(chunk.next().unwrap(), &"qux");
    assert_eq!(chunk.next(), None);

    let mut chunk = chunks.next().unwrap();
    assert_eq!(chunk.next().unwrap(), &"quux");
    assert_eq!(chunk.next(), None);

    assert!(chunks.next().is_none());
}

#[test]
fn interleave() {
    let input = vec![0..3, 4..7, 7..11];

    let expected = vec![0, 4, 7,
                        1, 5, 8,
                        2, 6, 9];

    let actual = input.into_iter().interleave().collect::<Vec<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn chain_all() {
    let input = vec![0..5, 5..10, 10..15];
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let actual = input.into_iter().chain_all().collect::<Vec<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn join() {
    let input = ["foo", "bar", "baz"];
    assert_eq!(input.iter().join(""),  "foobarbaz");
    assert_eq!(input.iter().join(","), "foo,bar,baz");
}

//------------------------------------------------------------------------------
struct Chunk<I: Iterator> {
    inner: I,
    size: usize
}

impl<I> Iterator for Chunk<I> where I: Iterator {
    type Item = ::std::vec::IntoIter<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: is there a way to avoid the collect and do it lazyly?
        let result = self.inner.by_ref().take(self.size).collect::<Vec<_>>();

        if result.is_empty() {
            return None;
        } else {
            return Some(result.into_iter());
        }
    }
}

//------------------------------------------------------------------------------
struct Interleave<I> {
    inner: Vec<I>,
    index: usize
}

impl<I> Iterator for Interleave<I> where I: Iterator {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.inner[self.index].next() {
            Some(e) => Some(e),
            None    => None
        };

        self.index = (self.index + 1) % self.inner.len();

        result
    }
}

//------------------------------------------------------------------------------
struct ChainAll<I: Iterator> {
    current: Option<I::Item>,
    all: I
}

impl<I> Iterator for ChainAll<I> where I: Iterator, I::Item: Iterator {
    type Item = <<I as Iterator>::Item as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bump = self.current.is_none();

        loop {
            if bump { self.current = self.all.next(); }

            match self.current {
                Some(ref mut current) => {
                    if let Some(result) = current.next() {
                        return Some(result);
                    } else {
                        bump = true;
                    }
                },
                None => return None
            }
        }
    }
}
