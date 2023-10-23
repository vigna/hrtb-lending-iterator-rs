/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![doc = include_str!("../README.md")]

mod adapters;
pub use self::adapters::from_into_iter;
pub use self::adapters::from_iter;

mod sources;
mod traits;

pub use self::traits::ExactSizeLendingIterator;
pub use self::traits::IntoLendingIterator;
pub use self::traits::Item;
pub use self::traits::LendingIterator;
pub use self::traits::LendingIteratorItem;

pub use self::traits::IntoIteratorExt;
pub use self::traits::IteratorExt;
pub use self::traits::SliceExt;

/// A macro to iterate easily over an [`IntoLendingIterator`].
///
/// The syntax makes it possible to write loops such as
/// ```ignore
/// for_lend!{x in into_iter =>
///     println!("{}", x);
/// }
/// ```
#[macro_export]
macro_rules! for_lend {
    ($var:ident in $iter:expr => $($tt:tt)*) => {
        let mut iter = $iter.into_lend_iter();
        while let Some($var) = iter.next() {
            $($tt)*
        }
    }
}

#[test]
fn test_macro() {
    // Mock impl
    struct Mock();

    impl Mock {
        pub fn iter(&self) -> MockLendingIterator {
            MockLendingIterator {}
        }
    }

    struct MockLendingIterator {}

    impl<'any> LendingIteratorItem<'any> for MockLendingIterator {
        type Type = &'any str;
    }

    impl LendingIterator for MockLendingIterator {
        fn next(&mut self) -> Option<Item<'_, Self>> {
            None
        }
    }

    fn test_mock_lend_iter(m: Mock) {
        for_lend! {x in m.iter() =>
            println!("{}", x);
        };
    }

    impl IntoLendingIterator for &Mock {
        type IntoLendIter = MockLendingIterator;
        fn into_lend_iter(self) -> Self::IntoLendIter {
            self.iter()
        }
    }

    fn test_mock_into_lend_iter(m: Mock) {
        for_lend! {x in &m =>
            println!("{}", x);
        };
    }

    test_mock_lend_iter(Mock {});
    test_mock_into_lend_iter(Mock {});
}
