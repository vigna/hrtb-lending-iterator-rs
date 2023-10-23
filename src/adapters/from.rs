/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::*;

#[derive(Clone, Debug)]
#[repr(transparent)]
/// A struct exposing an [`IntoIterator`] as a [`IntoLendingIterator`].
pub struct FromIntoIterator<I: IntoIterator>(I);

impl<I: IntoIterator> FromIntoIterator<I> {
    pub fn into_inner(self) -> I {
        self.0
    }
}

impl<'any, I: IntoIterator> LendingIteratorItem<'any> for FromIntoIterator<I> {
    type Type = I::Item;
}

impl<I: IntoIterator> IntoLendingIterator for FromIntoIterator<I> {
    type IntoLendIter = FromIterator<I::IntoIter>;

    fn into_lend_iter(self) -> Self::IntoLendIter {
        FromIterator(self.0.into_iter())
    }
}

/// Converts an [`Iterator`] into a [`LendingIterator`] without allocating.
///
/// This is always possible. Note that his operation and
/// [`LendingIterator::into_iter`] are mutually inverse.
///
/// This function can be more conveniently accessed using the
/// [`IteratorExt::into_lend_iter`] method.
pub fn from_iter<I: Iterator>(iter: I) -> FromIterator<I> {
    FromIterator(iter)
}

#[derive(Clone, Debug)]
#[repr(transparent)]
/// A struct exposing an [`Iterator`] as a [`LendingIterator`].
pub struct FromIterator<I: Iterator>(I);

impl<I: Iterator> FromIterator<I> {
    pub fn into_inner(self) -> I {
        self.0
    }
}

impl<T, I: Iterator<Item = T>> From<I> for FromIterator<I>
where
    for<'any> I: LendingIteratorItem<'any, Type = T>,
{
    fn from(iter: I) -> Self {
        FromIterator(iter.into_iter())
    }
}

impl<'any, I: Iterator> LendingIteratorItem<'any> for FromIterator<I> {
    type Type = I::Item;
}

impl<I: Iterator> LendingIterator for FromIterator<I> {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.0.next()
    }
}

/// Converts an [`IntoIterator`] into an [`IntoLendingIterator`] without allocating.
///
/// This is always possible. Note that his operation and
/// [`IntoLendingIterator::into_into_iter`] are mutually inverse.
///
/// This function can be more conveniently accessed using the
/// [`IntoIteratorExt::into_into_lend_iter`] method.

pub fn from_into_iter<I: IntoIterator>(iter: I) -> FromIntoIterator<I> {
    FromIntoIterator(iter)
}
