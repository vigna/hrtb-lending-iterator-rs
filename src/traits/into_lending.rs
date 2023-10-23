/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{
    adapters::{IntoIntoIter, ToIntoOwnedItemIterator},
    LendingIterator,
};

/**

A trait for types that can be turned into a [`LendingIterator`].

It plays the same role of [`IntoIterator`] for [`Iterator`], and
it has a corresponding blanket implementation for all types that
implement [`LendingIterator`].

Ideally, types that have a method that returns a [`LendingIterator`]
should implement this trait on a reference and delegate
[`IntoLendingIterator::into_lend_iter`] to such methods.

*/
pub trait IntoLendingIterator {
    /// Which kind of lending iterator are we turning this into?
    type IntoLendIter: LendingIterator;

    /// Create a lending iterator from a value.
    fn into_lend_iter(self) -> Self::IntoLendIter;

    /// Converts this into an [`IntoIterator`], if possible, without allocating.
    ///
    /// This method is only available if the items returned
    /// by the associated iterator are owned (i.e., if the iterator is
    /// not really lending).
    ///
    /// Note that this method and
    /// [`IntoIteratorExt::into_into_lend_iter`](crate::IntoIteratorExt::into_into_lend_iter) are
    /// mutually inverse.
    fn into_into_iter(self) -> IntoIntoIter<Self>
    where
        Self: Sized,
    {
        IntoIntoIter(self)
    }

    /// Turns this [`IntoLendingIterator`] into a regular [`IntoIterator`]
    /// by applying [`LendingIterator::to_owned_item`] to the result
    /// of [`IntoLendingIterator::into_lend_iter`].
    ///
    /// This method is only available if the type referred by
    /// the item type implements [`ToOwned`].
    fn to_into_owned_item(self) -> ToIntoOwnedItemIterator<Self>
    where
        Self: Sized,
    {
        ToIntoOwnedItemIterator(self)
    }
}

impl<I: LendingIterator> IntoLendingIterator for I {
    type IntoLendIter = Self;

    fn into_lend_iter(self) -> Self::IntoLendIter {
        self
    }
}
