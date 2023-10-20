/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::take_while`].
#[derive(Clone, Debug)]
pub struct TakeWhile<I: LendingIterator, F>
where
    F: FnMut(&'_ <I as LendingIteratorItem>::Type) -> bool,
{
    pub(crate) iter: I,
    pub(crate) predicate: F,
    pub(crate) ended: bool,
}

impl<'any, I: LendingIterator, F> LendingIteratorItem<'any> for TakeWhile<I, F>
where
    F: FnMut(&'_ <I as LendingIteratorItem>::Type) -> bool,
{
    type Type = <I as LendingIteratorItem<'any>>::Type;
}

impl<I, F> LendingIterator for TakeWhile<I, F>
where
    I: LendingIterator,
    F: FnMut(&'_ <I as LendingIteratorItem>::Type) -> bool,
{
    fn next(&mut self) -> Option<Item<'_, Self>> {
        if self.ended {
            None
        } else {
            let next_item = self.iter.next()?;
            if (self.predicate)(&next_item) {
                Some(next_item)
            } else {
                self.ended = true;
                None
            }
        }
    }
}
