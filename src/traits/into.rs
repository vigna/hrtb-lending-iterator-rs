/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::LendingIterator;

pub trait IntoLendingIterator {
    /// Which kind of iterator are we turning this into?
    type IntoIter: LendingIterator;

    fn into_lend_iter(self) -> Self::IntoIter;
}

impl<I: LendingIterator> IntoLendingIterator for I {
    type IntoIter = Self;

    fn into_lend_iter(self) -> Self::IntoIter {
        self
    }
}
