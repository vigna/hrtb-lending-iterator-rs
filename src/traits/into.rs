/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{LendingIterator, LendingIteratorItem};

pub trait IntoLendingIterator {
    type Item<'b>;
    /// Which kind of iterator are we turning this into?
    type IntoIter<'a>: LendingIterator + for<'b> LendingIteratorItem<'b, T = Self::Item<'b>>
    where
        Self: 'a;

    /// Creates an iterator from a value.
    fn into_lend_iter(self) -> Self::IntoIter<'static>;
}

pub trait IntoLendingIteratorMut: for<'a> LendingIteratorItem<'a> {
    type Item<'b>;
    /// Which kind of iterator are we turning this into?
    type IntoIter<'a>: LendingIterator + for<'b> LendingIteratorItem<'b, T = Self::Item<'b>>
    where
        Self: 'a;

    /// Creates an iterator from a value.
    fn into_lend_iter(&mut self) -> Self::IntoIter<'_>;
}
