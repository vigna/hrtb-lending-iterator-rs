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
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::Type),
{
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<'any, I: LendingIterator, F> LendingIteratorItem<'any> for Inspect<I, F>
where
    F: FnMut(&'_ <I as LendingIteratorItem>::Type),
{
    type Type = <I as LendingIteratorItem<'any>>::Type;
}

impl<I, F> LendingIterator for Inspect<I, F>
where
    I: LendingIterator,
    for<'any> F: FnMut(&'_ <I as LendingIteratorItem>::Type),
{
    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next().map(|item| {
            (self.f)(&item);
            item
        })
    }
}
