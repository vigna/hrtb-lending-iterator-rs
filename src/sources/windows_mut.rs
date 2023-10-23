/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{Item, LendingIterator, LendingIteratorItem};

/// This struct is returned by [`IteratorExt::windows_mut`].
pub struct WindowsMut<'a, T, const WINDOW_SIZE: usize> {
    pub(crate) slice: &'a mut [T],
    pub(crate) curr_pos: usize,
}

impl<'a, 'any, T, const WINDOW_SIZE: usize> LendingIteratorItem<'any>
    for WindowsMut<'a, T, WINDOW_SIZE>
{
    type Type = &'any mut [T; WINDOW_SIZE];
}

impl<'a, T, const WINDOW_SIZE: usize> LendingIterator for WindowsMut<'a, T, WINDOW_SIZE> {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        // See https://github.com/danielhenrymantilla/lending-iterator.rs/blob/5353b5e6ce8be9d07d0cfd86e23e481377074780/src/lending_iterator/constructors/windows_mut_.rs
        let window = self
            .slice
            .get_mut(self.curr_pos..)?
            .get_mut(..WINDOW_SIZE)?;
        self.curr_pos += 1;
        Some(window.try_into().unwrap())
    }
}
