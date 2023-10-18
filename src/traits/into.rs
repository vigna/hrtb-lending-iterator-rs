/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{LendingIterator, LendingIteratorItem};

pub trait IntoLendingIterator {
    /// The type of the elements being iterated over.
    type Item;
    /// Which kind of iterator are we turning this into?
    type IntoIter<'b>: LendingIterator + for<'a> LendingIteratorItem<'a, T = Self::Item>
    where
        Self: 'b;

    /// Creates an iterator from a value.
    fn into_lend_iter(&mut self) -> Self::IntoIter<'_>;
}
