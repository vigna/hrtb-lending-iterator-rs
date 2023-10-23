/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

mod take_while;
pub use take_while::TakeWhile;

mod into;
pub use into::IntoIntoIter;
pub use into::IntoIter;

mod inspect;
pub use inspect::Inspect;

mod from;
pub use from::from_into_iter;
pub use from::from_iter;
pub use from::FromIntoIterator;
pub use from::FromIterator;

mod enumerate;
pub use enumerate::Enumerate;

mod map;
pub use map::Map;

mod to_owned;
pub use to_owned::ToIntoOwnedItemIterator;
pub use to_owned::ToOwnedItemIterator;

/// Extension trait adding to [`IntoIterator`] the method [`into_into_lend_iter`](IntoIteratorExt::into_into_lend_iter),
/// which turns an [`IntoIterator`] into a [`IntoLendingIterator`](crate::IntoLendingIterator) without allocation.
#[extension(pub trait IntoIteratorExt)]
impl<I: IntoIterator> I {
    /// Turn this [`IntoIterator`] into a [`IntoLendingIterator`](crate::IntoLendingIterator) without allocation.
    ///
    /// Note that his method and
    /// [`IntoLendingIterator::into_into_iter`](crate::IntoLendingIterator::into_into_iter) are mutually inverse.
    fn into_into_lend_iter(self) -> FromIntoIterator<I> {
        from_into_iter(self)
    }
}

/// Extension trait adding to [`Iterator`] the method [`into_lend_iter`](IteratorExt::into_lend_iter),
/// which turns an [`Iterator`] into a [`LendingIterator`](crate::LendingIterator) without allocation.
#[extension(pub trait IteratorExt)]
impl<I: Iterator> I {
    /// Turn this [`Iterator`] into a [`LendingIterator`](crate::LendingIterator) without allocation.
    ///
    /// Note that his method and
    /// [`LendingIterator::into_iter`](crate::LendingIterator::into_iter) are mutually inverse.
    fn into_lend_iter(self) -> FromIterator<I> {
        from_iter(self)
    }
}
