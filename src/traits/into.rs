/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{LendingIterator, LendingIteratorItem};

pub trait IntoLendingIterator: for<'a> LendingIteratorItem<'a> {
    /// Which kind of iterator are we turning this into?
    type IntoIter<'a>: LendingIterator
    where
        Self: 'a;

    /// Creates an iterator from a value.
    fn into_lend_iter(&mut self) -> Self::IntoIter<'_>;
}
