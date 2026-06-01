use std::{cmp::Reverse, collections::BinaryHeap};

use crate::search::score::structures::scored_line::ScoredLine;

// Uses the [std::cmp::Reverse] wrapper
pub struct SearchResultItemHolder<'a, const N: usize>(BinaryHeap<Reverse<ScoredLine<'a>>>);

impl<'a, const N: usize> SearchResultItemHolder<'a, N> {
    pub fn new() -> Self {
        SearchResultItemHolder(BinaryHeap::with_capacity(N))
    }

    pub fn extend<I: Iterator<Item = ScoredLine<'a>>>(&mut self, iter: I) {
        self.0.extend(iter.map(|item| Reverse(item)));
        while self.0.len() > N {
            self.0.pop();
        }
    }

    pub fn items(self) -> Vec<ScoredLine<'a>> {
        let as_vec = self.0.into_sorted_vec();
        as_vec.into_iter().map(|i| i.0).collect()
    }
}
