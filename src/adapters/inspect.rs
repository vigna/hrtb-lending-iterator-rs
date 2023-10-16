/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::inspect`]
#[derive(Clone, Debug)]
pub struct Inspect<I: LendingIterator, F>
where
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::T),
{
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<'succ, I: LendingIterator, F> LendingIteratorItem<'succ> for Inspect<I, F>
where
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::T),
{
    type T = <I as LendingIteratorItem<'succ>>::T;
}

impl<I, F> LendingIterator for Inspect<I, F>
where
    I: LendingIterator,
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::T),
{
    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next().map(|item| {
            (self.f)(&item);
            item
        })
    }
}
