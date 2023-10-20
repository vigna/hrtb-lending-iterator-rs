/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{LendingIterator, LendingIteratorItem};
use std::borrow::ToOwned;

/// This struct is returned by [`LendingIterator::into_owned`].
#[derive(Clone, Debug)]
pub struct IntoOwned<I: LendingIterator>(pub(crate) I);

impl<Item: ?Sized + ToOwned, I: LendingIterator> Iterator for IntoOwned<I>
where
    I: for<'any> LendingIteratorItem<'any, Type = &'any Item>,
{
    type Item = Item::Owned;

    fn next(&mut self) -> Option<Item::Owned> {
        self.0.next().map(|x| x.to_owned())
    }
}
