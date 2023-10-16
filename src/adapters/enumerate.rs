/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::enumerate`]
#[derive(Clone, Debug)]
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}
impl<I> Enumerate<I> {
    pub fn new(iter: I) -> Enumerate<I> {
        Enumerate { iter, count: 0 }
    }
}

impl<'succ, I: LendingIterator> LendingIteratorItem<'succ> for Enumerate<I> {
    type T = (usize, <I as LendingIteratorItem<'succ>>::T);
}

impl<I: LendingIterator> LendingIterator for Enumerate<I> {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
        Some((i, a))
    }
}
