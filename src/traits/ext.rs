/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{adapters::FromIntoIterator, adapters::FromIterator, sources::WindowsMut};

/// Extension trait adding to [`IntoIterator`] the method [`into_into_lend_iter`](IntoIteratorExt::into_into_lend_iter),
/// which turns an [`IntoIterator`] into a [`IntoLendingIterator`](crate::IntoLendingIterator) without allocation.
pub trait IntoIteratorExt<I: IntoIterator> {
    /// Turn this [`IntoIterator`] into a [`IntoLendingIterator`](crate::IntoLendingIterator) without allocation.
    ///
    /// Note that his method and
    /// [`IntoLendingIterator::into_into_iter`](crate::IntoLendingIterator::into_into_iter) are mutually inverse.
    fn into_into_lend_iter(self) -> FromIntoIterator<I>;
}

impl<I: IntoIterator> IntoIteratorExt<I> for I {
    fn into_into_lend_iter(self) -> FromIntoIterator<I> {
        crate::from_into_iter(self)
    }
}

/// Extension trait adding to [`Iterator`] the method [`into_lend_iter`](IteratorExt::into_lend_iter),
/// which turns an [`Iterator`] into a [`LendingIterator`](crate::LendingIterator) without allocation.
pub trait IteratorExt<I: Iterator + Sized> {
    /// Turn this [`Iterator`] into a [`LendingIterator`](crate::LendingIterator) without allocation.
    ///
    /// Note that his method and
    /// [`LendingIterator::into_iter`](crate::LendingIterator::into_iter) are mutually inverse.
    fn into_lend_iter(self) -> FromIterator<I>;
}

impl<I: Iterator> IteratorExt<I> for I {
    fn into_lend_iter(self) -> FromIterator<I> {
        crate::from_iter(self)
    }
}

/// Extension trait adding to slices the method
/// [`windows_mut`](SliceExt::windows_mut), which is like
/// [`windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows), but yields a
/// lending iterator returning mutable references to arrays.
pub trait SliceExt<T> {
    /// Like [`windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows),
    /// but yields a lending iterator returning mutable references to arrays.
    fn windows_mut<const WINDOW_SIZE: usize>(&mut self) -> WindowsMut<'_, T, WINDOW_SIZE>;
}

impl<T> SliceExt<T> for [T] {
    fn windows_mut<const WINDOW_SIZE: usize>(&mut self) -> WindowsMut<'_, T, WINDOW_SIZE> {
        WindowsMut {
            slice: self,
            curr_pos: 0,
        }
    }
}
