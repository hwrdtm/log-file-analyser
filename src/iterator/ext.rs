use std::fmt::Display;

pub struct Printer<I>
where
    I: Iterator,
{
    iter: I,
}

impl<I> Iterator for Printer<I>
where
    I: Iterator,
    I::Item: Display,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            Some(item) => {
                println!("{}", item);
                Some(item)
            }
            None => None,
        }
    }
}

impl<I> Printer<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Printer { iter }
    }
}

#[derive(Clone)]
pub struct MergedStream<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
{
    this_iter: I,
    other_iter: I,
    f: F,

    this_item: Option<I::Item>,
    other_item: Option<I::Item>,
}

pub fn merge<I, F>(this_iter: I, other_iter: I, f: F) -> MergedStream<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
{
    MergedStream {
        this_iter,
        other_iter,
        f,

        this_item: None,
        other_item: None,
    }
}

impl<I, F> Iterator for MergedStream<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Always try to fill the internal state of this_item and other_item first before comparing which to return.
        if self.this_item.is_none() {
            self.this_item = self.this_iter.next();
        }
        if self.other_item.is_none() {
            self.other_item = self.other_iter.next();
        }

        // Now that we have attempted to fill the internal state, we can compare the two items and return one.
        if self.this_item.is_some() && self.other_item.is_some() {
            let this_item = self.this_item.as_ref().unwrap();
            let other_item = self.other_item.as_ref().unwrap();

            if (self.f)(this_item, other_item) {
                // Return this item and clear the storage.
                let ret = this_item.to_owned();
                self.this_item = None;

                Some(ret)
            } else {
                // Return other item and clear the storage.
                let ret = other_item.to_owned();
                self.other_item = None;

                Some(ret)
            }
        } else if self.this_item.is_some() {
            let ret = self.this_item.clone();
            self.this_item = None;

            ret
        } else if self.other_item.is_some() {
            let ret = self.other_item.clone();
            self.other_item = None;

            ret
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        arr1: Vec<usize>,
        arr2: Vec<usize>,
        expected: Vec<usize>,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                arr1: vec![1, 3, 5],
                arr2: vec![2, 4, 6],
                expected: vec![1, 2, 3, 4, 5, 6],
            },
            TestCase {
                arr1: vec![1, 2, 3],
                arr2: vec![4, 5, 6],
                expected: vec![1, 2, 3, 4, 5, 6],
            },
            TestCase {
                arr1: vec![1, 2, 3],
                arr2: vec![],
                expected: vec![1, 2, 3],
            },
            TestCase {
                arr1: vec![],
                arr2: vec![],
                expected: vec![],
            },
            TestCase {
                arr1: vec![1, 2, 3],
                arr2: vec![1, 2, 3],
                expected: vec![1, 1, 2, 2, 3, 3],
            },
            TestCase {
                arr1: vec![1, 2, 3],
                arr2: vec![1, 2, 3, 4, 5, 6],
                expected: vec![1, 1, 2, 2, 3, 3, 4, 5, 6],
            },
            TestCase {
                arr1: vec![1, 4, 5, 6],
                arr2: vec![2, 3, 7, 8],
                expected: vec![1, 2, 3, 4, 5, 6, 7, 8],
            },
        ]
    }

    #[test]
    fn merging_iterator_collect() {
        let test_cases = test_cases();

        for (idx, test_case) in test_cases.iter().enumerate() {
            // For each test case we need to check both directions of merging.
            let merged = merge(
                test_case.arr1.clone().into_iter(),
                test_case.arr2.clone().into_iter(),
                |a, b| a < b,
            );
            let collected: Vec<usize> = merged.collect();
            assert_eq!(collected, test_case.expected, "test case {}", idx);

            let merged = merge(
                test_case.arr2.clone().into_iter(),
                test_case.arr1.clone().into_iter(),
                |a, b| a < b,
            );
            let collected: Vec<usize> = merged.collect();
            assert_eq!(collected, test_case.expected, "test case {}", idx);
        }
    }

    #[test]
    fn merging_iterators_next() {
        let test_cases = test_cases();

        for (idx, test_case) in test_cases.iter().enumerate() {
            // For each test case we need to check both directions of merging.
            let mut merged = merge(
                test_case.arr1.clone().into_iter(),
                test_case.arr2.clone().into_iter(),
                |a, b| a < b,
            );
            for expected in &test_case.expected {
                assert_eq!(merged.next(), Some(*expected), "test case {}", idx);
            }

            let mut merged = merge(
                test_case.arr2.clone().into_iter(),
                test_case.arr1.clone().into_iter(),
                |a, b| a < b,
            );
            for expected in &test_case.expected {
                assert_eq!(merged.next(), Some(*expected), "test case {}", idx);
            }
        }
    }
}
