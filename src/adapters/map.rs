/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`LendingIterator::map`]
#[derive(Clone, Debug)]
pub struct Map<I: LendingIterator, F, NewItemType>
where
    for<'any> F: FnMut(<I as LendingIteratorItem>::T) -> NewItemType,
{
    pub(crate) iter: I,
    pub(crate) map: F,
}

impl<'succ, I: LendingIterator, NewItemType, F> LendingIteratorItem<'succ>
    for Map<I, F, NewItemType>
where
    for<'any> F: FnMut(<I as LendingIteratorItem>::T) -> NewItemType,
{
    type T = NewItemType;
}

impl<I, NewItemType, F> LendingIterator for Map<I, F, NewItemType>
where
    I: LendingIterator,
    for<'any> F: FnMut(<I as LendingIteratorItem>::T) -> NewItemType,
{
    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.iter.next().map(|item| (self.map)(item))
    }
}