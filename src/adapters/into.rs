/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{IntoLendingIterator, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::into_iter`].
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct IntoIter<I: LendingIterator>(pub(crate) I);

impl<I: LendingIterator> IntoIter<I> {
    pub fn into_inner(self) -> I {
        self.0
    }
}

impl<Item, I: LendingIterator> Iterator for IntoIter<I>
where
    for<'any> I: LendingIteratorItem<'any, Type = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        self.0.next()
    }
}

/// This struct is returned by [`IntoLendingIterator::into_into_iter`].
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct IntoIntoIter<I: IntoLendingIterator>(pub(crate) I);

impl<I: IntoLendingIterator> IntoIntoIter<I> {
    pub fn into_inner(self) -> I {
        self.0
    }
}

impl<Item, I: IntoLendingIterator> IntoIterator for IntoIntoIter<I>
where
    for<'any> I::IntoLendIter: LendingIteratorItem<'any, Type = Item>,
{
    type Item = Item;
    type IntoIter = IntoIter<I::IntoLendIter>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_lend_iter())
    }
}
