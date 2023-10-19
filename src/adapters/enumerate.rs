/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::enumerate`]
#[derive(Clone, Debug)]
pub struct Enumerate<I> {
    pub(crate) iter: I,
    pub(crate) count: usize,
}

impl<I> Enumerate<I> {
    pub fn new(iter: I) -> Enumerate<I> {
        Enumerate { iter, count: 0 }
    }
}

impl<'any, I: LendingIterator> LendingIteratorItem<'any> for Enumerate<I> {
    type Type = (usize, <I as LendingIteratorItem<'any>>::Type);
}

impl<I: LendingIterator> LendingIterator for Enumerate<I> {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
        Some((i, a))
    }
}
