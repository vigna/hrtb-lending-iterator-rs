/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{IntoLendingIterator, LendingIterator, LendingIteratorItem};
use std::borrow::ToOwned;

#[derive(Clone, Debug)]
/// This struct is returned by [`LendingIterator::to_owned`].
pub struct ToOwnedItemIterator<I: LendingIterator>(pub(crate) I);

impl<Item: ?Sized + ToOwned, I: LendingIterator> Iterator for ToOwnedItemIterator<I>
where
    I: for<'any> LendingIteratorItem<'any, Type = &'any Item>,
{
    type Item = Item::Owned;

    fn next(&mut self) -> Option<Item::Owned> {
        self.0.next().map(|x| x.to_owned())
    }
}

#[derive(Clone, Debug)]
/// This struct is returned by [`IntoLendingIterator::to_into_owned`].
pub struct ToIntoOwnedItemIterator<I: IntoLendingIterator>(pub(crate) I);

impl<Item: ?Sized + ToOwned, I: IntoLendingIterator> IntoIterator for ToIntoOwnedItemIterator<I>
where
    I::IntoLendIter: for<'any> LendingIteratorItem<'any, Type = &'any Item>,
{
    type Item = Item::Owned;
    type IntoIter = ToOwnedItemIterator<I::IntoLendIter>;

    fn into_iter(self) -> Self::IntoIter {
        LendingIterator::to_owned(self.0.into_lend_iter())
    }
}
