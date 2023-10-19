/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![doc = include_str!("../README.md")]

mod adapters;
mod traits;

pub use self::adapters::Enumerate;
pub use self::adapters::Inspect;
pub use self::adapters::IntoIter;
pub use self::adapters::TakeWhile;
pub use self::traits::ExactSizeLendingIterator;
pub use self::traits::IntoLendingIterator;
pub use self::traits::Item;
pub use self::traits::LendingIterator;
pub use self::traits::LendingIteratorItem;

#[macro_export]
macro_rules! for_lend {
    ($var:ident in $iter:expr => $($tt:tt)*) => {
        let mut iter = $iter.into_iter();
        while let Some($var) = iter.next() {
            $($tt)*
        }
    }
}
