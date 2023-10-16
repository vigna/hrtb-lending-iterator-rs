/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::map`]
#[derive(Clone, Debug)]
pub struct TakeWhile<I: LendingIterator, F>
where
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::T) -> bool,
{
    pub(crate) iter: I,
    pub(crate) predicate: F,
    pub(crate) ended: bool,
}

impl<'succ, I: LendingIterator, F> LendingIteratorItem<'succ> for TakeWhile<I, F>
where
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::T) -> bool,
{
    type T = <I as LendingIteratorItem<'succ>>::T;
}

impl<I, F> LendingIterator for TakeWhile<I, F>
where
    I: LendingIterator,
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::T) -> bool,
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
