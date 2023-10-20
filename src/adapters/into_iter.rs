/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::into_iter`].
#[derive(Clone, Debug)]
pub struct IntoIter<I: ?Sized + LendingIterator>(pub I);

impl<Item, I: ?Sized + LendingIterator> Iterator for IntoIter<I>
where
    for<'any> I: LendingIteratorItem<'any, Type = Item>,
{
    type Item = Item;

    fn next(self: &'_ mut IntoIter<I>) -> Option<Item> {
        self.0.next()
    }
}
