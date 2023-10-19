/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::LendingIterator;

/**

A trait for types that can be turned into a [`LendingIterator`].

It plays the same role of [`IntoIterator`] for [`Iterator`], and
it has a corresponding blanket implementation for all types that
implement [`LendingIterator`].

Ideally, types that have a method that returns a [`LendingIterator`]
should implement this trait on a reference and delegate [`IntoLendingIterator::into_lend_iter`]
to such methods.

*/
pub trait IntoLendingIterator {
    /// Which kind of lending iterator are we turning this into?
    type IntoLendIter: LendingIterator;

    fn into_lend_iter(self) -> Self::IntoLendIter;
}

impl<I: LendingIterator> IntoLendingIterator for I {
    type IntoLendIter = Self;

    fn into_lend_iter(self) -> Self::IntoLendIter {
        self
    }
}
